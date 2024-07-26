mod args;
mod state;
mod topology;

use crate::{
    state::{RequestedRoom, RoomId, ServerState},
    topology::MatchmakingDemoTopology,
};
use args::Args;
use axum::{http::StatusCode, response::IntoResponse, routing::get,  Json};
use clap::Parser;
use matchbox_signaling::SignalingServerBuilder;
use tracing::info;
use tracing_subscriber::prelude::*;
use tee_attestation::{get_quote, guess_tee, TeeType};
use serde_json::json;


fn setup_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "matchbox_server=info,tower_http=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_file(false)
                .with_target(false),
        )
        .init();
}

#[tokio::main]
async fn main() {
  

    setup_logging();
    let args = Args::parse();

    // Setup router
    info!("Matchbox Signaling Server: {}", args.host);

    let mut state = ServerState::default();
    let server = SignalingServerBuilder::new(args.host, MatchmakingDemoTopology, state.clone())
        .on_connection_request({
            let mut state = state.clone();
            move |connection| {
                let room_id = RoomId(connection.path.clone().unwrap_or_default());
                let next = connection
                    .query_params
                    .get("next")
                    .and_then(|next| next.parse::<usize>().ok());
                let room = RequestedRoom { id: room_id, next };
                state.add_waiting_client(connection.origin, room);
                Ok(true) // allow all clients
            }
        })
        .on_id_assignment({
            move |(origin, peer_id)| {
                info!("Client connected {origin:?}: {peer_id:?}");
                state.assign_id_to_waiting_client(origin, peer_id);
            }
        })
        .cors()
        .trace()
        .mutate_router(|router| router.route("/health", get(health_handler)))
        .mutate_router(|router| router.route("/attestation", get(attestation_handler)))

        .build();
    server
        .serve()
        .await
        .expect("Unable to run signaling server, is it already running?")
}

pub async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn attestation_handler() -> impl IntoResponse {
    let tee_type = guess_tee().unwrap();
    let raw_quote = get_quote(None).unwrap();

    match tee_type {
        TeeType::AzSev | TeeType::Sev => {
            let quote = sev_quote::quote::parse_quote(&raw_quote).unwrap();
            println!(
                "AMD SEV-SNP found:\n{}",
                 quote.report
            );
            return (StatusCode::OK, Json(json!({"quote": raw_quote}))).into_response();

        }
        TeeType::Tdx | TeeType::AzTdx => {
            let (quote, _) = tdx_quote::quote::parse_quote(&raw_quote).unwrap();
            println!("Intel TDX found:\n{}", quote);
            return (StatusCode::OK, Json(json!({"quote": raw_quote}))).into_response();

        }
        TeeType::Sgx => {
            let (quote, _, _, _) = sgx_quote::quote::parse_quote(&raw_quote).unwrap();
            println!("Intel SGX found:\n{}", quote);
            return (StatusCode::OK, Json(json!({"quote": raw_quote}))).into_response();
        }
    }

}
