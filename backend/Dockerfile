FROM rustlang/rust:nightly-slim as builder
RUN apt update && apt install -y libsqlite3-dev
WORKDIR /app
ADD db_layer/ db_layer/
ADD web_layer/ web_layer/
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM debian:10-slim
RUN apt update && apt install -y sqlite
WORKDIR /app
COPY --from=builder /app/target/* ./
COPY crud.db crud.db
COPY .env .env
RUN chmod a+x web_layer
CMD [ "/app/web_layer" ]