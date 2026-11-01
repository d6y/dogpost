# Dogpost

Converts an email into a blog post.

Reads an IMAP email account, and commits a Hugo-style markdown post and images files to Github.

# License

Apache 2.0

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


# Running as a Home Assistant add-on

The `homeassistant/` folder contains a local add-on for Home Assistant.

The add-on stays running and runs `dogpost` once each time an automation sends it a line of input.
Each run processes one email.

Install steps from macOS:

## 1. Expose the Home Assistant `addons` folder to your Mac

1. In Home Assistant go to _Settings > Apps_, and install **Samba share**.
2. On the add-on's _Configuration_ tab set a username and password, save, then **Start** it.
3. On the Mac, in Finder choose _Go > Connect to Server_ (⌘K) and enter `smb://homeassistant.local/addons` (or use IP address if that doesn't work).
   Log in with the Samba username and password. The share mounts at `/Volumes/addons`.
4. Check you can access the addons from your terminal: `ls /Volumes/addons`. If you can't check _System Settings > Privacy & Security > Files and Folders_ and for your terminal all turn on "Network Volumes".

## 2. Copy the add-on across

From the root of this repository:

```
./homeassistant/install.sh
```

This copies the add-on files plus `Cargo.toml`, `Cargo.lock` and `src/` into `/Volumes/addons/dogpost`.
Pass a different folder as an argument if the share is mounted elsewhere.

## 3. Install the add-on

1. In Home Assistant go to _Settings > Apps_ and click **Install App**. Select `dogpost` and then press  **Install**.

The Pi will now compile the code and this will take a while.

## 4. Configure

On the add-on's **Configuration** tab fill in all the settings!

Save, then **Start** the add-on. The **Log** tab should show `dogpost: waiting for triggers`.

## 5. Trigger it from an automation

Use the `hassio.addon_stdin` action (the add-on's id is `local_dogpost`).
For example, with the IMAP integration watching the mailbox:

```yaml
alias: Dogpost on new email
triggers:
  - trigger: event
    event_type: imap_content
    event_data:
      initial: true
actions:
  - action: hassio.addon_stdin
    data:
      addon: local_dogpost
      input: run
mode: queued
```

To test by hand, go to _Developer tools > Actions_, run `hassio.addon_stdin` with the data above, and watch the add-on's **Log** tab.

## Updating

After changing the code, run `./homeassistant/install.sh` again, press the "refresh" icon on the Apps page in HA, then select **Update** on the app's page.

After changing `homeassistant/dogpost/config.yaml` (e.g., the settings), Rebuild is not enough:

1. Increase `version` in `config.yaml`.
2. Run `./homeassistant/install.sh`.
3. In the Home Assistant _Terminal_:

   ```
   ha store reload
   ha apps update local_dogpost
   ```
