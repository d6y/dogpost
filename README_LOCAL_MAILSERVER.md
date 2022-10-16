# Testing via Greenmail

Oct 2022: this isn't quite working any more :-(

## Running Greenmail server via docker

NB: As of Oct 2022, this no longer works as the certificates for TLS do not seem to be accepted by Thunderbird.

```
$ docker run -ti -e GREENMAIL_OPTS='-Dgreenmail.setup.test.all -Dgreenmail.hostname=0.0.0.0 -Dgreenmail.auth.disabled -Dgreenmail.verbose' -p 3025:3025 -p 3110:3110 -p 3143:3143 -p 3465:3465 -p 3993:3993 -p 3995:3995 greenmail/standalone:1.5.9
```

## Running Greenmail with custom certificates

Thank you: <https://crispinstichart.github.io/using-SSL-in-greenmail-docker-container/>

```
/usr/local/opt/openssl/bin/openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 -nodes \
  -keyout greenmail.key -out greenmail.crt \
  -subj "/CN=localhost" -addext "subjectAltName=DNS:localhost,IP:127.0.0.1"

/usr/local/opt/openssl/bin/openssl pkcs12 \
  -export -out greenmail.p12 -inkey greenmail.key -in greenmail.crt
```

Then [download the standalone JAR](https://greenmail-mail-test.github.io/greenmail/#download), and run it:

```
java -Dgreenmail.auth.disabled \
  -Dgreenmail.tls.keystore.file=./greenmail.p12 -Dgreenmail.tls.keystore.password=woof \
  -Dgreenmail.setup.test.all -Dgreenmail.hostname=127.0.0.1 -Dgreenmail.verbose \
  -jar greenmail-standalone-1.6.11.jar
 ```

## Run dogpost

Run dogpost with:

```
SSL_CERT_FILE=./greenmail.crt AWS_ACCESS_KEY_ID=zzz AWS_SECRET_ACCESS_KEY=yyy \
  cargo run -- --imap-password dog --imap-hostname localhost --imap-port 3993 \
  --imap-user dog@127.0.0.1 --posts-dir ./tmp --media-dir ./tmp --s3-bucket xxx
```

As of Oct 2022, I'm unable to get rustls to accept self-generated certificates inside the imap client (`invalid peer certificate: UnknownIssuer`).


## Thunderbird set up

![IMAP Settings](etc/tb-imap.png)

![SMTP Setting](etc/tb-smtp.png)

![Sending an email](etc/tb-send.png)


