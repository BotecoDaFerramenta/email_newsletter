#! Dockerfile
# docker build --tag email_newsletter --file Dockerfile . && docker run -p 1991:8000 email_newsletter
FROM rust:1.89

WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE=true
ENV RUST_BACKTRACE=1
ENV APP_ENVIRONMENT=production
RUN cargo build --release
ENTRYPOINT [ "./target/release/email_newsletter" ]
