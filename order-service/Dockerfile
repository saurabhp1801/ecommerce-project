FROM rustlang/rust:nightly-slim AS builder  

WORKDIR /app

# Copy and prepare dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo +nightly build --release
RUN rm -rf src

# Copy project files and build
COPY . .
RUN cargo +nightly build --release

# Final stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/order-service .

CMD ["./order-service"]
