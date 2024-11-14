ARG RUST_VERSION="1.79"
ARG RUST_IMAGE="rust:${RUST_VERSION}-bullseye"
ARG BASE_IMAGE="debian:bullseye"

# -- Install --
# We only pay the installation cost once,
# it will be cached from second build onwards
FROM ${RUST_IMAGE} as install

RUN apt-get -y update && apt-get -y upgrade
RUN apt-get install -y make git zlib1g-dev libssl-dev gperf php-cli cmake g++

# Install cargo chef
RUN cargo install cargo-chef --locked
RUN mkdir build

WORKDIR /app

# -- Cache for builder --
FROM install as planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# -- Builder with cache layer --
FROM install as builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --workspace --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --workspace

# -- Base Image --
# Base image for running tests and production image
FROM ${BASE_IMAGE} as base

RUN apt-get update && apt-get install -y ca-certificates

# -- Production Image --
FROM base as prod
WORKDIR /app
COPY --from=builder /app/target/release/graphql /app/graphql
COPY --from=install /usr/local/lib/libtd* /usr/local/lib/
RUN ldconfig
COPY ./.env ./.env

WORKDIR /app

# -- Run supervisord --
CMD ["/usr/bin/supervisord"]