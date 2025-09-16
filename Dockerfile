FROM ubuntu:25.10@sha256:a61c057b4f69200ecf031519a20db79b8683837ba1dc2a59458d333eb75b174d AS builder

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


FROM ubuntu:25.10@sha256:a61c057b4f69200ecf031519a20db79b8683837ba1dc2a59458d333eb75b174d AS runner

ENV LANG=C.UTF-8

ARG USER_NAME=electrix
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

# Setup the environment and copy the compiled binary
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    if ! getent passwd ${USER_UID} > /dev/null; then \
        groupadd --gid ${USER_GID} ${USER_NAME} && \
        useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USER_NAME}; \
    fi && \
    mkdir -p /app/data && \
    chown ${USER_UID}:${USER_GID} /app/data
USER ${USER_UID}
WORKDIR /app
COPY --from=builder /app/target/release/electrix-api /app/electrix-api

ENTRYPOINT [ "/app/electrix-api" ]
CMD [ "2025-01-01" ]
