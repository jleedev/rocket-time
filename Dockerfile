FROM rustlang/rust:nightly as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/lib.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/lib.rs && rmdir src

COPY src src
RUN cargo build --release

FROM debian:stable-slim

WORKDIR /app
COPY --from=builder /app/target/release/rocket-time .
EXPOSE 8000
ENV ROCKET_ENV=stage
CMD ["./rocket-time"]
