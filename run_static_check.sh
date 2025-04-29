#!/bin/bash

# Fail on error
set -e


function run_tests {
    # Analyze the code
    # ================
    echo "Checking the code with cargo check"
    cargo check --release

    echo "Running tests with cargo test"
    cargo test --release

    # Run static analysis with cargo clippy
    # =====================================
    echo "Running static analysis with cargo clippy"
    cargo clippy --release -- -D warnings -W clippy::all -W clippy::pedantic -W clippy::nursery

    # Run dependency check with cargo deny
    # ====================================
    if [ -z "$(cargo --list | grep deny)" ]
    then
        # install cargo deny
        echo "Cargo deny is not installed. Install with: cargo install --locked cargo-deny"
    else
        echo "Running dependency check with cargo deny"
        cargo deny --all-features --log-level error check
    fi
}

function echo_instructions {
    echo "Usage: $0 (--local|--docker)"
    exit 1
}


if [ -z "$1" ]
then
    echo_instructions
fi

if [ "$1" == "--local" ]
then
    run_tests
    exit 0
fi

if [ "$1" == "--docker" ]
then
    docker build --tag "mx-electric-api:check" --file "Dockerfile-check" .
    docker run --rm "mx-electric-api:check"
    exit 0
fi

# invalid options
echo_instructions
