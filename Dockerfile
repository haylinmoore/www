FROM debian:bookworm-slim

ENV TZ="America/New_York"
ARG REF=""
ARG COMMIT=""
ARG TIME=""

ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}

COPY target/release/www /usr/local/bin/
COPY ./assets /usr/local/bin/assets
COPY ./content /usr/local/bin/content

WORKDIR /usr/local/bin
EXPOSE 3000
CMD ["/usr/local/bin/www"]
