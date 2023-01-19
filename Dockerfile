# Build backend
FROM rust:1.65.0 AS backend
WORKDIR /app
COPY . .

RUN apt-get update
RUN apt-get install -y lld clang
RUN cargo install --path .

# Build frontend
FROM node:19 AS frontend
WORKDIR /app
COPY . .
WORKDIR ./frontend

RUN npm install
RUN npm run build

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=backend /app/target/release/birdnest .
COPY --from=backend /app/config ./config
COPY --from=frontend /app/dist ./dist
ENTRYPOINT ["/app/birdnest"]
