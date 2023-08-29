FROM rust:1.72.0 as build
WORKDIR /usr/src/rust_web

# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build to cache dependencies
RUN mkdir ./src && echo 'fn main() { }' > ./src/main.rs
RUN cargo build --release

# Remove dummy source files and copy actual source files over
RUN rm -rf ./src
COPY ./src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs
RUN cargo build --release


# copy built files to lightweight google image expose port and run
FROM gcr.io/distroless/cc-debian12
COPY --from=build /usr/src/rust_web/target/release/rust_web /usr/local/bin/
WORKDIR /usr/local/bin
EXPOSE 8080
CMD ["rust_web"]
