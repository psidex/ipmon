# ipmon

[![client-ci](https://github.com/psidex/ipmon/actions/workflows/client.yml/badge.svg)](https://github.com/psidex/ipmon/actions/workflows/client.yml)
[![server-ci](https://github.com/psidex/ipmon/actions/workflows/server.yaml/badge.svg)](https://github.com/psidex/ipmon/actions/workflows/server.yaml)

Monitor your public IP and get notified when it changes.

## Client (ipmon)

The CLI app which monitors a machines public IP by periodically requesting it from a web server on the internet, such as [checkip.amazonaws.com](https://checkip.amazonaws.com/). Alternatively you can deploy your own server like this using [ipmon-server](#server-ipmon-server).

If an IP change is detected, notifications can be sent via most services by using [apprise](https://github.com/caronc/apprise).

Currently the `psidex/ipmon` Docker image is tailored for ARM64 so it can run on a Raspberry Pi.

### Deploy Using Docker

```bash
cd ~
git clone https://github.com/psidex/ipmon.git
cd /your/working/directory
cp ~/ipmon/example-config.yaml config.yaml
vi config.yaml
rm -rf ~/ipmon
docker run -d --name ipmon \
    --restart unless-stopped \
    -v $(pwd)/config.yaml:/app/config.yaml \
    psidex/ipmon:latest
```

### Config

ipmon is configured using a `config.yaml` file, this should exist in the working directory, or the path can be provided as an arg, e.g. `ipmon ./path/to/config.yaml`.

For the Docker image, this config file should be mounted to the internal path `/app/config.yaml`. If you want access to the ip cache file, you can mount a file to `/app/ipmon.cache`.

Use the `server` key to set the IP web server, this can be an [ipmon-server](#server-ipmon-server) instance, or any other URL that returns a **plaintext IPv4**, ipmon currently does not work with IPv6.

The `interval` key is the number of seconds ipmon will wait between checking for an IP change.

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

If behind a reverse proxy, make sure it's configured to set the [X-Forwarded-For](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For) header, as the server uses [this method](https://docs.rs/actix-web/4.3.1/actix_web/dev/struct.ConnectionInfo.html#method.realip_remote_addr) to get the "real" IP. I recommend [Caddy](https://caddyserver.com/) which does this automatically.

The server provides 2 endpoints, `/` (for your IP) and `/health` (returns 200 with an empty body).

If you are hosting ipmon-server behind a domain name, to ensure the server always returns a v4 IP and not v6, you should **not** set a [AAAA DNS record](https://www.cloudflare.com/en-gb/learning/dns/dns-records/dns-aaaa-record/).

### Deploy Using Docker

```bash
docker run -d --name ipmon-server \
    --restart unless-stopped \
    -p 8080:8080 \
    psidex/ipmon-server:latest
```
