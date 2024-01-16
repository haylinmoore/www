# Build stage
FROM rust:1.75-bullseye as builder

# Set arguments and environment variables
ARG REF=""
ARG COMMIT=""
ARG TIME=""
ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}
ENV CT=${CT}
ENV TZ="America/New_York"

# Copy the source code
ADD . .

# Build the project
RUN cargo build --release

# Final stage
FROM debian:bullseye

ARG REF=""
ARG COMMIT=""
ARG TIME=""
ENV COMMIT=${COMMIT}
ENV REF=${REF}
ENV TIME=${TIME}
ENV CT=${CT}
ENV TZ="America/New_York"


# Copy the binary from the build stage
COPY --from=builder /target/release/www /usr/local/bin/www

COPY ./assets /usr/local/bin/assets
COPY ./content /usr/local/bin/content

# Set the command to run the binary
WORKDIR /usr/local/bin
CMD ["www"]
EXPOSE 3000
