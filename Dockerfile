FROM rust:1.70

ARG REF=""
ARG COMMIT=""
ARG TIME=""

ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}
ENV CT=${CT}

ADD . .

RUN cargo build --release

CMD ["./target/release/www2"]
