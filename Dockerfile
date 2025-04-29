FROM ubuntu:25.04@sha256:79efa276fdefa2ee3911db29b0608f8c0561c347ec3f4d4139980d43b168d991 AS builder

WORKDIR /app
ENV LANG=C.UTF-8

# Install the latest stable version of Rust and cargo
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y curl gcc libssl-dev openssl pkg-config && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    apt-get clean
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the code
COPY . .

# Compile the code
RUN cargo build --release


FROM ubuntu:25.04@sha256:79efa276fdefa2ee3911db29b0608f8c0561c347ec3f4d4139980d43b168d991 AS runner

ENV LANG=C.UTF-8

# Setup the environment and copy the compiled binary
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd -g 1234 electrix && \
    useradd -m -u 1234 -g electrix electrix && \
    mkdir -p /app/data && \
    chown electrix:electrix /app/data
USER electrix
WORKDIR /app
COPY --from=builder /app/target/release/electrix-api /app/electrix-api

ENTRYPOINT [ "/app/electrix-api" ]
CMD [ "2025-01-01" ]
