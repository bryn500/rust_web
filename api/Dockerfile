FROM rust:1.72.0 as build
WORKDIR /app

# Copy Cargo files
COPY ./Cargo.toml .

# Create fake main.rs file in src and build
# this caches the downloading of dependencies unless they change
# the cache will not be invalidated if main is changed
RUN mkdir ./src && echo 'fn main() { }' > ./src/main.rs
RUN cargo build --release

# Remove dummy source files and copy actual source files over
RUN rm -rf ./src
COPY ./src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs
RUN cargo build --release


# copy built files to lightweight google image expose port and provide start command
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=build /app/target/release/api .
EXPOSE 8080
CMD ["./api"]
