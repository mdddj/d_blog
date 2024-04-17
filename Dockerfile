FROM rust:1.67 as builder
WORKDIR /app
COPY ./ /app
RUN cargo build --release --verbose
RUN strip target/release/d_blog
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/d_blog .
COPY ./config/config.yml /app/config/
COPY ./data/demo.db /app/data/
ENTRYPOINT ["/d_blog"]