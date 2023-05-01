FROM rust:latest as builder
RUN USER=root cargo new --bin os_signals
WORKDIR /os_signals

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# compile dependencies in earlier layer for caching
RUN cargo build --release
RUN rm src/*.rs && rm ./target/release/deps/os_signals* 

COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc

ENV MODE=Sync

COPY --from=builder /os_signals/target/release/os_signals /

CMD ["./os_signals"]

STOPSIGNAL SIGTERM