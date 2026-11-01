#!/bin/sh
# Home Assistant add-on entry point.
# Reads add-on options, then runs dogpost once for every line received on stdin
# (sent by the hassio.addon_stdin action from an automation).
set -eu

OPTIONS="${OPTIONS:-/data/options.json}"

opt() { jq -r ".$1" "$OPTIONS"; }

export IMAP_HOSTNAME="$(opt imap_hostname)"
export IMAP_PORT="$(opt imap_port)"
export IMAP_USER="$(opt imap_user)"
export IMAP_PASSWORD="$(opt imap_password)"
export MAILBOX="$(opt mailbox)"
export EXPURGE="$(opt expunge)"
export GITHUB_TOKEN="$(opt github_token)"
export GITHUB_REPO="$(opt github_repo)"
export GITHUB_BRANCH="$(opt github_branch)"
export GITHUB_POST_PATH="$(opt github_post_path)"
export TZ="$(opt timezone)"
export RUST_LOG="$(opt log_level)"

# Media paths are passed as arguments: both settings share the
# GITHUB_MEDIA_PATH environment variable in settings.rs.
set -- \
  --github-media-path "$(opt github_media_path)" \
  --web-media-path "$(opt web_media_path)"

for domain in $(jq -r '.allowed_domains[]' "$OPTIONS"); do
  set -- "$@" --allowed-domains "$domain"
done

echo "dogpost: waiting for triggers"
while read -r _; do
  echo "dogpost: triggered"
  dogpost "$@" || echo "dogpost: failed (exit $?)"
done
