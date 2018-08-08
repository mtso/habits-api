FROM debian:jessie-slim

WORKDIR /app

COPY Rocket.toml .

COPY build/release/token-service .

CMD /app/token-service
