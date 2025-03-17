#!/bin/bash

# Fail on error
set -e


if [ ! -z "$2" ] && [ "$1" == "--local" ]
then
    # Run the application locally
    set -o allexport
    source .env
    set +o allexport
    cargo build --release
    ./target/release/electrix-api "$2"
elif [ ! -z "$2" ] && [ "$1" == "--docker" ]
then
    # Run the application in a docker container
    docker build --tag "mx-electric-api:latest" --file "Dockerfile" .
    docker stop electrix-api > /dev/null 2>&1 || true
    docker rm electrix-api > /dev/null 2>&1 || true
    docker run --rm --env-file ".env" --volume "$(pwd)/data:/app/data:rw" --name "electrix-api" "mx-electric-api:latest" "$2"
else
    echo "Usage: $0 (--local|--docker) <DATE>"
    exit 1
fi
