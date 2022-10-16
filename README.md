# Dogpost

Converts an email into a blog post.

Reads an IMAP email account, writes attachments to S3, and commits a Jekyll-style markdown post file to Github.

## How use use

You need to supply:

- paths to write the markdown blog post and image files
- Google email address and password
- S3 bucket name, key, secret (as environment variables only)
- Github personal token, repository (user, branch)

The subject is used as the title of the blog post and the filename. 

## Serving suggestion

```
AWS_ACCESS_KEY_ID=??? AWS_SECRET_ACCESS_KEY=??? cargo run -- \
  --imap-password 1234 --imap-user you@example.org \
  --media-dir ./tmp --s3-bucket static.example.com \
  --github-token ??? --github-repo user/repo --github-branch main \
  --github-path _posts \
  --expurge
```

NB: `--expurge` will archive/delete the email after processing.

# Docker build

```
docker build -t dogpost .
docker run -it --rm --name running-dogpost dogpost
```

# License

Apache 2.0

