FROM rust:1.70

ARG REF=""
ARG COMMIT=""
ARG TIME=""

ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}
ENV CT=${CT}
ENV TZ="America/New_York"

ADD . .

RUN cargo build --release

CMD ["./target/release/www"]
