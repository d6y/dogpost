Telepost
========

Reads an IMAP email account and writes JPEG attachments to the file system and creates a Markdown file for Telegr.am or Jekyll.

How use use
-----------

You need to supply:

- path to write the markdown blog post to
- path to write image files
- Google email adrress
- Address password
- S3 bucket name
- S3 key
- S3 secret

For example:

The subject is used as the title of the blog post and the filename.

It will then delete the email (archive it).

Serving suggestion
------------------

Run via cron, and wrapper in a script that either commits and pushes to your GitHub hosted telegr.am blog, or move to a Dropbox folder.


Testing via Greenmail
=====================

Run a local SMTP/IMAP server:

```
$ docker run -t -i -e GREENMAIL_OPTS='-Dgreenmail.setup.test.all -Dgreenmail.hostname=0.0.0.0 -Dgreenmail.auth.disabled -Dgreenmail.verbose' -p 3025:3025 -p 3110:3110 -p 3143:3143 -p 3465:3465 -p 3993:3993 -p 3995:3995 greenmail/standalone:1.5.9
```

Set up something like Thunderbird to send mail to local SMTP and collect from local IMAP.

Run dogpost with:

```
cargo run -- --imap-allow-untrusted --imap-password 1234 --imap-hostname 127.0.0.1 --imap-port 3993 --imap-user dog@127.0.0.1 --posts-dir ./tmp --media-dir ./tmp --s3-bucket xxx --s3-key yyy --s3-secret zzz
```

License
=======

Apache 2.0

