FROM lukemathwalker/cargo-chef:0.1.67-rust-alpine3.19 AS planner
WORKDIR /app
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:0.1.67-rust-alpine3.19 AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/src/target cargo build --release --target x86_64-unknown-linux-musl 

FROM alpine AS runtime

ARG REF=""
ARG COMMIT=""
ARG TIME=""

ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}
ENV CT=${CT}
ENV TZ="America/New_York"

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/www /usr/local/bin/
COPY ./assets /usr/local/bin/assets
COPY ./content /usr/local/bin/content

WORKDIR /usr/local/bin
CMD ["/usr/local/bin/www"]
EXPOSE 3000