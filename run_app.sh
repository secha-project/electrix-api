#!/bin/bash

# Fail on error
set -e


if [ ! -z "$2" ] && [ "$1" == "--local" ]
then
    COMPILE_TARGET=./target/release/electrix-api
    if [ ! -f ${COMPILE_TARGET} ]
    then
        bash compile_app.sh --local
    fi

    # Run the application locally
    set -o allexport
    source .env
    set +o allexport
    ${COMPILE_TARGET} $2 $3

elif [ ! -z "$2" ] && [ "$1" == "--docker" ]
then
    DOCKER_IMAGE="mx-electric-api:latest"
    if [[ "$(docker images -q ${DOCKER_IMAGE} 2> /dev/null)" == "" ]]
    then
        bash compile_app.sh --docker
    fi

    if [ ! -z "$3" ]
    then
        TARGET_PATH="$3"
    else
        TARGET_PATH="$(pwd)/data"
    fi

    # Run the application in a docker container
    docker stop electrix-api > /dev/null 2>&1 || true
    docker rm electrix-api > /dev/null 2>&1 || true
    docker run --rm \
        --env-file ".env" \
        --volume "${TARGET_PATH}:/app/data:rw" \
        --name "electrix-api" \
        ${DOCKER_IMAGE} $2 /app/data

else
    echo "Usage: $0 (--local|--docker) <DATE> [<TARGET_PATH>]"
    exit 1
fi
