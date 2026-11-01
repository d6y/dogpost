# Home Assistant add-on image.
# Build context is the add-on folder, assembled by homeassistant/install.sh
# (add-on files plus Cargo.toml, Cargo.lock and src/).
FROM alpine:3.22 AS builder

RUN apk add --no-cache \
    rust \
    cargo \
    musl-dev \
    gcc

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.22 AS runtime

RUN apk add --no-cache ca-certificates imagemagick imagemagick-heic imagemagick-jpeg ffmpeg jq tzdata
COPY --from=builder /app/target/release/dogpost /usr/local/bin/dogpost
COPY run.sh /run.sh
RUN chmod a+x /run.sh

CMD ["/run.sh"]
