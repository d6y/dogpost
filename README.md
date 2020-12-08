Dogpost
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


License
=======

Apache 2.0

