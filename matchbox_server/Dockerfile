#  webrtc relay with multi-tee attestation
#
# to build, run `docker build -f matchbox_server/Dockerfile` from root of the
# repository

FROM rust:1.79-slim-bullseye AS builder

WORKDIR /usr/src/matchbox_server/

COPY README.md /usr/src/README.md
COPY matchbox_server/Cargo.toml /usr/src/matchbox_server/Cargo.toml
COPY matchbox_protocol /usr/src/matchbox_protocol
COPY matchbox_server /usr/src/matchbox_server
COPY matchbox_signaling /usr/src/matchbox_signaling
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    libssl-dev \
    tpm2-tools \
    libtss2-dev \
    pkg-config \
    llvm-dev \
    libclang-dev \
    make \
    build-essential \
    wget \
    python-is-python3 \
    debhelper \
    zip \
    libcurl4-openssl-dev \
    libboost-dev \
    libboost-system-dev \
    libboost-thread-dev \
    protobuf-c-compiler \
    libprotobuf-c-dev \
    protobuf-compiler \
    libc6 \
    git


RUN wget https://download.01.org/intel-sgx/latest/dcap-latest/linux/distro/Debian12/sgx_linux_x64_sdk_2.24.100.3.bin
RUN chmod +x sgx_linux_x64_sdk_2.24.100.3.bin
RUN yes '/opt/intel' | ./sgx_linux_x64_sdk_2.24.100.3.bin
RUN . /opt/intel/sgxsdk/environment
RUN git clone https://github.com/intel/SGXDataCenterAttestationPrimitives
RUN cd SGXDataCenterAttestationPrimitives/; git submodule update --init --recursive
RUN ./SGXDataCenterAttestationPrimitives/QuoteGeneration/download_prebuilt.sh
RUN make -C ./SGXDataCenterAttestationPrimitives/QuoteGeneration/
RUN make -C ./SGXDataCenterAttestationPrimitives/QuoteGeneration deb_pkg;
RUN dpkg -i --force-overwrite SGXDataCenterAttestationPrimitives/QuoteGeneration/installer/linux/deb/*.deb

RUN cargo build --release


FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/matchbox_server/target/release/matchbox_server /usr/local/bin/matchbox_server
CMD ["matchbox_server"]