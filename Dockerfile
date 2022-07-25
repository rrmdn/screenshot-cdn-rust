FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin screenshot-cdn-rust
WORKDIR ./screenshot-cdn-rust
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm target/x86_64-unknown-linux-musl/release/screenshot-cdn-rust*

ADD . ./
RUN cargo build --release

FROM zenika/alpine-chrome:latest as release

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=chrome

COPY --from=builder /home/rust/src/screenshot-cdn-rust/target/x86_64-unknown-linux-musl/release/screenshot-cdn-rust /usr/src/app/screenshot-cdn-rust

RUN chown -R $APP_USER:$APP_USER /usr/src/app

USER $APP_USER
RUN ls -lah

ENTRYPOINT ["./screenshot-cdn-rust"]
