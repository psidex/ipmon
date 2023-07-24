# ipmon

[![client-ci](https://github.com/psidex/ipmon/actions/workflows/client.yml/badge.svg)](https://github.com/psidex/ipmon/actions/workflows/client.yml)
[![server-ci](https://github.com/psidex/ipmon/actions/workflows/server.yaml/badge.svg)](https://github.com/psidex/ipmon/actions/workflows/server.yaml)

## Client (ipmon)

An app to monitor a machines public IP, providing notifications via [apprise](https://github.com/caronc/apprise).

Currently the Dockerfile and GitHub action are tailored for an ARM64 build (specifically to run on a Raspberry Pi).

```bash
cd ~
git clone https://github.com/psidex/ipmon.git
cd /your/working/directory
cp ~/ipmon/example-config.yaml config.yaml
vi config.yaml
rm -rf ~/ipmon
touch ipmon.cache
docker run -d --name ipmon \
    --restart unless-stopped \
    -v $(pwd)/config.yaml:/app/config.yaml \
    -v $(pwd)/ipmon.cache:/app/ipmon.cache \
    psidex/ipmon:latest
```

### Config

ipmon is configured using a `config.yaml` file, this should exist in the same directory as the binary, or the path can be provided as an arg, e.g. `ipmon ./path/to/config.yaml`.

For the Docker image, the config shuold be mounted to the internal path `/app/config.yaml`. If you want access to the ip cache file, you can mount an empty file to `/app/ipmon.cache`.

Use the `server` key to set the IP server, this can be an ipmon-server instance, or any other URL that returns a **plaintext IPv4**.

The `interval` key is the number of seconds between fetching the IP.

If the `notifications` key is set, it should contain an array of objects, each with a `title`, `body`, and `url`. These will be passed into [apprise](https://github.com/caronc/apprise), which **will need to be installed and in your PATH if you're using ipmon as a standalone binary** (i.e. not the Docker image).

Example config:

```yaml
server: "https://checkip.amazonaws.com/"
interval: 600
notifications:
  - title: ""
    body: "ipmon - IP changed to %s"
    url: "twilio://abc:def@123/456"

```

## Server (ipmon-server)

A web server that replies with the clients IP.

This will work as a standalone webserver or behind a reverse proxy.

If behind a reverse proxy, make sure it's configured to set the [X-Forwarded-For](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For) header, as the server uses [this method](https://docs.rs/actix-web/4.3.1/actix_web/dev/struct.ConnectionInfo.html#method.realip_remote_addr) to get the "real" IP. I recommend [Caddy](https://caddyserver.com/) which will do this automatically.

The server provides 2 endpoints, `/` (for your IP) and `/health` (returns 200 with an empty body).

```bash
docker run -d --name ipmon-server \
    --restart unless-stopped \
    -p 8080:8080 \
    psidex/ipmon-server:latest
```
