FROM lukemathwalker/cargo-chef as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build project deps and not the app
RUN cargo chef cook --release --recipe-path recipe.json
# Upto this point if all the deps remain the same, all the layers should be cached
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin rust-microservice

FROM gcr.io/distroless/cc AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/rust-microservice rust-microservice
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./rust-microservice"]