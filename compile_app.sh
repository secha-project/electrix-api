#!/bin/bash

# Fail on error
set -e


if [ "$1" == "--local" ]
then
    # Compile the application locally
    cargo build --release
elif [ "$1" == "--docker" ]
then
    # Build the Docker image for the application
    docker build \
        --build-arg "USER_UID=$(id -u)" \
        --tag "mx-electric-api:latest" \
        --file "Dockerfile" .
else
    echo "Usage: $0 (--local|--docker)"
    exit 1
fi
