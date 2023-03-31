# Dogpost

Converts an email into a blog post.

Reads an IMAP email account, and commits a Hugo-style markdown post and images files to Github.

## How use use

```
dogpost [OPTIONS] --imap-user <IMAP_USER> --imap-password <IMAP_PASSWORD> --github-token <GITHUB_TOKEN> --github-repo <GITHUB_REPO>
```

For more clues:

```
cargo run -- --help
```

NB: `--expurge` will archive/delete the email after processing.

# Docker build

```
docker build -t dogpost .
docker run -it --rm --name running-dogpost dogpost
```

# License

Apache 2.0

