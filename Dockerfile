FROM rust:1.64.0-bullseye as cargo
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim as rt
RUN apt-get update && apt-get install -y --no-install-recommends imagemagick
RUN apt-get install -y --no-install-recommends ca-certificates
COPY --from=cargo /usr/local/cargo/bin/dogpost /usr/local/bin/dogpost
ENV TZ="Europe/London"
CMD ["dogpost"]
