FROM rust:1.57 as build

# Create a new project
RUN USER=root cargo new --bin shopify-developer-challenge
WORKDIR ./shopify-developer-challenge

# Install dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && rm src/*.rs

# Copy source files
ADD ./build.rs ./build.rs
ADD ./src ./src
ADD ./migrations ./migrations
ADD ./sqlx-data.json ./sqlx-data.json

# Build application
RUN rm ./target/release/deps/shopify_developer_challenge*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

# Install system dependencies
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Set the default environment
ENV TZ=Etc/UTC \
    APP_USER=app \
    ADDRESS=0.0.0.0:3000
EXPOSE 3000

# Run as non-root user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p $APP

# Install the app
COPY --from=build --chown=$APP_USER:$APP_USER /shopify-developer-challenge/target/release/shopify-developer-challenge ${APP}/shopify-backend-challenge

# Setup to run the app
USER $APP_USER
WORKDIR ${APP}
CMD ["./shopify-backend-challenge"]
