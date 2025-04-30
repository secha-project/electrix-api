# Data fetcher for MX Electric API

The current functionality of the application is to fetch data from the MX Electric API and store the data in CSV files.
Currently, the application fetches one full day of data for a specific date for all available devices.

The application has been tested with Rust version 1.86.0 on Ubuntu environment.

Contents:

- [Running static checks](#running-static-checks)
- [Running the application](#running-the-application)

## Running static checks

The static checks can be run either locally or in a Docker container.

### Running static checks locally

Requires Rust installation with the `cargo deny` tool.

On Ubuntu, these can be installed with the following commands:

```bash
    sudo apt-get update
    sudo apt-get install -y curl gcc libssl-dev openssl pkg-config
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
    cargo install --locked cargo-deny
```

To run the static checks locally, run the following command:

```bash
./run_static_check.sh --local
```

### Running static checks in a Docker container

Requires Docker installation: [https://docs.docker.com/engine/install/ubuntu/](https://docs.docker.com/engine/install/ubuntu/)

To run the static checks in a Docker container, run the following command:

```bash
./run_static_check.sh --docker
```

## Running the application

The application can be run either locally or in a Docker container.

### Running the application locally

To run the application locally requires a Rust installation. To run the application locally, run the following command:

```bash
./run_app.sh --local <DATE>
```

where `<DATE>` is the date to fetch data for in the format `YYYY-MM-DD`.

The application will write the data to the `data` directory.

### Running the application in a Docker container

To run the application in a Docker container, run the following command:

```bash
./run_app.sh --docker <DATE>
```

where `<DATE>` is the date to fetch data for in the format `YYYY-MM-DD`.

The application will write the data to the `data` directory.
