# syntax=docker/dockerfile:1

####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev capnproto
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

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/basin_worker /bin/

# Use an unprivileged user.
USER basin:basin

EXPOSE 3000
EXPOSE 3001

CMD ["/bin/basin_worker"]
LABEL service=worker
