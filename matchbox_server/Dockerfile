#  webrtc relay with multi-tee attestation
#
# to build, run `docker build -f matchbox_server/Dockerfile` from root of the
# repository

FROM  ubuntu:22.04 AS builder
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
RUN apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev python3 python3-pip libusb-1.0-0 libssl-dev \
    pkg-config libtinfo5

WORKDIR /usr/src/matchbox_server/
COPY README.md /usr/src/README.md
COPY matchbox_server/Cargo.toml /usr/src/matchbox_server/Cargo.toml
COPY matchbox_protocol /usr/src/matchbox_protocol
COPY matchbox_server /usr/src/matchbox_server
COPY matchbox_signaling /usr/src/matchbox_signaling
RUN  apt-get update &&  apt-get upgrade &&   apt-get install -y \
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
    libc6-dev \ 
    git


RUN git clone https://github.com/intel/linux-sgx
RUN make -C linux-sgx/ sdk
RUN make -C linux-sgx/ psw
run mace -C linux-sgx/ install
RUN git clone https://github.com/intel/SGXDataCenterAttestationPrimitives
RUN cd SGXDataCenterAttestationPrimitives/; git submodule update --init --recursive
RUN ./SGXDataCenterAttestationPrimitives/QuoteGeneration/download_prebuilt.sh
RUN make -C ./SGXDataCenterAttestationPrimitives/
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