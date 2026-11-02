FROM rust
WORKDIR /app
RUN rustup component add rustfmt
COPY . .
RUN ./scripts/check.sh
CMD ["cargo", "run"]
