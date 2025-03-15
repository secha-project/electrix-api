#!/bin/bash

# Fail on error
set -e


if [ ! -z "$2" ] && [ "$1" == "--local" ]
then
    # Run the application locally
    host_url=$(cat .env | grep HOST_URL= | cut -d'=' -f2)
    access_token=$(cat .env | grep ACCESS_TOKEN= | cut -d'=' -f2)
    allow_invalid_certificates=$(cat .env | grep ALLOW_INVALID_CERTIFICATES= | cut -d'=' -f2)

    cargo build --release
    HOST_URL=${host_url} \
        ACCESS_TOKEN=${access_token} \
        ALLOW_INVALID_CERTIFICATES=${allow_invalid_certificates} \
        ./target/release/electrix-api "$2"
elif [ ! -z "$2" ] && [ "$1" == "--docker" ]
then
    # Run the application in a docker container
    docker build --tag "mx-electric-api:latest" --file "Dockerfile" .
    docker run --rm --env-file ".env" "mx-electric-api:latest" "$2"
else
    echo "Usage: $0 (--local|--docker) <DATE>"
    exit 1
fi
