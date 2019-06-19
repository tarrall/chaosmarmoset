#
# This is supposed to be a tiny tool, so I've put a little extra effort into
# making the Docker container small as well -- should be under 10 MB thanks
# to being "FROM scratch".  In order to do that, we need to use the musl builder
# to get a static binary.
#
FROM ekidd/rust-musl-builder AS builder

# Create a new empty shell project, copy our manifests over, and build the
# project... so that we can cache all our built crates.  Speeds up build times
# a bunch.
ENV USER=rust
WORKDIR /home/rust/src
RUN cargo new --bin project
WORKDIR /home/rust/src/project
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN sudo chown -R rust:rust ./
RUN cargo build --release
RUN rm src/*.rs
RUN rm -f ./target/x86_64-unknown-linux-musl/release/deps/chaosmarmoset*

# Build our project for reals
COPY ./src ./src
RUN sudo chown -R rust:rust ./
RUN cargo build --release

# Could use this later if we need ssl certs or such:
# FROM gcr.io/distroless/base
FROM scratch
COPY --from=builder /home/rust/src/project/target/x86_64-unknown-linux-musl/release/chaosmarmoset /usr/local/bin/chaosmarmoset
ENTRYPOINT ["chaosmarmoset"]
