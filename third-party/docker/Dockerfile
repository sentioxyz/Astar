FROM docker.io/library/ubuntu:22.04 AS builder

# Branch or tag to build moonbeam from
ARG COMMIT="master"
ARG RUSTFLAGS=""
ENV RUSTFLAGS=$RUSTFLAGS
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /

RUN echo "*** Installing Basic dependencies ***"
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
RUN apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler pkg-config

RUN set -e

RUN echo "*** Installing Rust environment ***"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup update
RUN rustup default stable

# Copy the entire project
ADD . astar

# Set working directory
WORKDIR /astar/astar

# Print target cpu
RUN rustc --print target-cpus

# Build the project with evm-tracing features
RUN cargo build --profile=production --all --features=evm-tracing

# Clone astar-runtime-overrides
RUN git clone --depth=1 https://github.com/sentioxyz/astar-runtime-overrides /astar-runtime-overrides

# Final stage
FROM ubuntu:22.04

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.parity.image.authors="devops@stake.co.jp" \
	io.parity.image.vendor="Stake Technologies" \
	io.parity.image.title="AstarNetwork/Astar" \
	io.parity.image.description="Astar: " \
	io.parity.image.source="https://github.com/AstarNetwork/Astar/blob/${VCS_REF}/third-party/docker/Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://docs.astar.network"

# show backtraces
ENV RUST_BACKTRACE 1

# add user
RUN useradd -m -u 1000 -U -s /bin/sh -d /astar astar && \
   	mkdir /data && \
    	chown -R astar:astar /data

ARG PROFILE=release
# add binary to docker image
COPY --from=builder /astar/target/production/astar-collator /usr/local/bin

COPY --from=builder /astar-runtime-overrides/astar /astar-runtime-overrides/astar

USER astar 

# check if executable works in this container
RUN ["astar-collator", "--version"]

EXPOSE 30333 30334 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/astar-collator","-d","/data"]