Telepost
========

Reads an IMAP email account and writes JPEG attachments to the file system and creates a Markdown file for Telegr.am or Jekyll.

How use use
-----------

The main method expects the following arguments:

- path to write the markdown blog post to
- Google email adrress
- Address password
- S3 bucket name
- S3 key
- S3 secret

For example:

    sbt "runMain Main blog/_posts me@example.org mypassw0rd images.bucket xxx yyy"

or:

    $ sbt assembly
    $ java -jar target/scala-2.11/telepost-assembly-1.0.1.jar blog/_post email pass bucket key secret

The subject is used as the title of the blog post and the filename.

It will then delete the email (archive it).


Serving suggestion
------------------

Run via cron, and wrapper in a script that either commits and pushes to your GitHub hosted telegr.am blog, or move to a Dropbox folder.


Testing via Greenmail
=====================

```
 docker run -t -i -p 3025:3025 -p 3110:3110 -p 3143:3143 \
                 -p 3465:3465 -p 3993:3993 -p 3995:3995 \
                 greenmail/standalone:1.6.0
```

Set up something like Thunderbird to send mail to local SMTP and collect from local IMAP. TODO: figure out TLS/etc

Run this code with something like:

```
cargo run -- --imap-password 1234 --imap-hostname 127.0.0.1 --imap-port 3143 --imap-user dog@127.0.0.1 --posts-dir ./pt --s3-bucket xxx --s3-key yyy --s3-secret zzz
```


License
=======

Apache 2.0

Contains code from https://github.com/hoisted/hoisted
