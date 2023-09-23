FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /rust-ci-cd-app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /rust-ci-cd-app/target/x86_64-unknown-linux-musl/release/rust-ci-cd-app /rust-ci-cd-app
ENTRYPOINT ["/rust-ci-cd-app"]
EXPOSE 3000