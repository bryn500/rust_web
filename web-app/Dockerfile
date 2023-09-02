# Stage 1: build rust app
FROM rust:1.72.0 as build
WORKDIR /app

COPY ./Cargo.toml .

# Create a dummy project to cache dependency downloads
RUN mkdir ./src && \
    echo 'fn main() { println!("Dummy") }' > src/main.rs && \
    cargo build --release && \
    rm -rf ./src

COPY ./src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs

# Build the release version from src
RUN cargo build --release

# Stage 2: Build Node.js Application
FROM node:20-slim as node-build
WORKDIR /app

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

# Stage 3: Build Node.js Application
FROM node-build AS node-output

COPY package.json pnpm-lock.yaml ./
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

COPY static/ static/
COPY templates/ templates/
COPY tailwind.config.js ./
RUN pnpm run build

# Stage 4: Use a lightweight base image for the final image
FROM gcr.io/distroless/cc-debian12
#FROM debian:latest 

WORKDIR /app

# Copy the built executable and assets
COPY --from=build /app/target/release/web .
COPY --from=node-output /app/static/ ./static
COPY --from=node-output /app/templates/ ./templates

# RUN ls && echo "----"

EXPOSE 8080

CMD ["./web"]
