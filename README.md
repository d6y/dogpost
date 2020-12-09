# Dogpost

Reads an IMAP email account and writes attachments to S3 and creates a Markdown file for Telegr.am or Jekyll.

## How use use

You need to supply:

- path to write the markdown blog post to
- path to write image files
- Google email adrress
- Address password
- S3 bucket name
- S3 key
- S3 secret

The subject is used as the title of the blog post and the filename. It will then delete the email (archive it).

## Serving suggestion

Run via cron, and wrapper in a script that either commits and pushes to your GitHub hosted telegr.am blog, or move to a Dropbox folder.

```
AWS_ACCESS_KEY_ID=??? AWS_SECRET_ACCESS_KEY=?? cargo run -- \
  --imap-password 1234 --imap-user you@example.org \
  --posts-dir ./tmp --media-dir ./tmp \
  --s3-bucket static.skitters.dallaway.com
  --expurge
```

NB: `--expurge` will archive/delete the email after processing.


# Building for Linux on macOS

```
docker pull clux/muslrust
docker run -v $PWD:/volume -t clux/muslrust cargo build --release
```

The binary will be:

```
target/x86_64-unknown-linux-musl/release/dogpost
```

# License

Apache 2.0

