# Birdnest

Hosted at https://weathered-dust-3635.fly.dev

## Build instructions

The application may be either built locally or with docker.
By default, the app is hosted on port 3000.

1. [Local](#local)
2. [Docker](#docker)

### Local
To build & run locally, you need working [rust](rustup.rs) and
[node](https://nodejs.org/en/download/package-manager/) installations.

1. `git clone https://github.com/impliedfeline/birdnest.git`
2. `cd frontend && npm install && npm run build && cd ..`
3. `cargo run`

### Docker
To build & run with docker:

1. `docker build --tag birdnest`
2. `docker run -p 3000:3000 birdnest`

## Tests

To run tests, run `cargo test`
