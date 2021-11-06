FROM rust as build

WORKDIR /repo

COPY Cargo.lock .
COPY Cargo.toml .
COPY cargo.config /usr/local/cargo/config


RUN mkdir src
RUN echo "// empty file" > src/lib.rs
# RUN cargo build --release

COPY src src


RUN cargo install --path .


FROM alpine

WORKDIR /app


expose 8000
expose 80

COPY --from=build /usr/local/cargo/bin/url-shorten /usr/local/bin/url-shorten 

COPY Rocket.toml .
CMD ["url-shorten"]
