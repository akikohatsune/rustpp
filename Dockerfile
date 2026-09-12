# Multi-stage, multi-platform Dockerfile for rustpp (oppai)
# Supports linux/amd64, linux/arm64, linux/arm/v7
FROM --platform=$BUILDPLATFORM rust:alpine AS builder

WORKDIR /usr/src/rustpp
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release && cp target/release/oppai /oppai

FROM alpine:latest
COPY --from=builder /oppai /usr/local/bin/oppai
ENTRYPOINT ["oppai"]
CMD ["-"]
