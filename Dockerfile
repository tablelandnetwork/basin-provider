# syntax=docker/dockerfile:1

####################################################################################################
## Builder
####################################################################################################
ARG RUST_VERSION=latest
FROM rust:${RUST_VERSION} AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y ca-certificates musl-tools musl-dev capnproto
RUN update-ca-certificates

# Create appuser
ENV USER=basin
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --locked --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch
ARG CRATE

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/$CRATE /bin/app

# Use an unprivileged user.
USER basin:basin

EXPOSE 3000
EXPOSE 3001

ENTRYPOINT ["/bin/app"]
