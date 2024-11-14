ARG RUST_VERSION="1.79"
ARG RUST_IMAGE="rust:${RUST_VERSION}-bullseye"
ARG BASE_IMAGE="debian:bullseye"

# -- Install --
# We only pay the installation cost once,
# it will be cached from second build onwards
FROM ${RUST_IMAGE} as install

RUN apt-get -y update && apt-get -y upgrade
RUN apt-get install -y make git zlib1g-dev libssl-dev gperf php-cli cmake g++
WORKDIR /app

# -- Builder with cache layer --
FROM install as builder
COPY . .
RUN cargo build --release --workspace

# -- Base Image --
# Base image for running tests and production image
FROM ${BASE_IMAGE} as base

RUN apt-get update && apt-get install -y ca-certificates

# -- Production Image --
FROM base as prod
WORKDIR /app
COPY --from=builder /app/target/release/financial-scraper /app/financial-scraper
COPY --from=builder /app/.env /app/.env
RUN ldconfig

# -- Copy migrations folder -
COPY ./migrations /app/migrations
# -- Run supervisord --
ENTRYPOINT ["/app/financial-scraper"]