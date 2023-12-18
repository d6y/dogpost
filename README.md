# Dogpost

Converts an email into a blog post.

Reads an IMAP email account, and commits a Hugo-style markdown post and images files to Github.

## How use use

```
dogpost [OPTIONS] --imap-user <IMAP_USER> --imap-password <IMAP_PASSWORD> --github-token <GITHUB_TOKEN> --github-repo <GITHUB_REPO>
```

NB: 

- `--expurge` will archive/delete the email after processing.
- `--dry-run` will not commit files to Git, but will print information about file locations.


Requires https://imagemagick.org to be installed.

# Docker build

```
docker build -t dogpost .
docker run -it --rm --name running-dogpost dogpost
```

# License

Apache 2.0

