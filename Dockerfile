FROM clux/muslrust:stable as builder
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN mkdir /app
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /app/target/x86_64-unknown-linux-musl/release/rindexer_cli

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rindexer_cli /app/rindexer

ENTRYPOINT ["/app/rindexer"]