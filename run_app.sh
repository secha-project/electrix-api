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
    docker run --rm --env-file ".env" --volume "$(pwd)/data:/app/data:rw" "mx-electric-api:latest" "$2"
else
    echo "Usage: $0 (--local|--docker) <DATE>"
    exit 1
fi
