# ipmon

[![client-ci](https://github.com/psidex/ipmon/actions/workflows/client.yml/badge.svg)](https://github.com/psidex/ipmon/actions/workflows/client.yml)
[![server-ci](https://github.com/psidex/ipmon/actions/workflows/server.yaml/badge.svg)](https://github.com/psidex/ipmon/actions/workflows/server.yaml)

A super simple app and web server to monitor my LANs public IP and notify me of a renewal using a forwardable text.

## Client

Currently the Dockerfile and GitHub action are tailored for an ARM64 build (to run on my Raspberry Pi 4B).

```bash
cd ~
git clone https://github.com/psidex/ipmon.git
cd ipmon
docker build -t psidex/ipmon:latest Dockerfile.client
cd /your/working/directory
cp ~/ipmon/ipmon.env .
vi ipmon.env
touch ipmon.cache
docker run -d --name ipmon \
    --restart unless-stopped \
    --env-file $(pwd)/ipmon.env \
    -v $(pwd)/ipmon.cache:/app/ipmon.cache \
    psidex/ipmon:latest
```

## Server

A web server that replies with the value of one of (in order of enumeration):
- X-Real-Ip
- X-Forwarded-For
- Connection IP

```bash
cd ~
git clone https://github.com/psidex/ipmon.git
cd ipmon
docker build -t psidex/ipmon-server:latest Dockerfile.server
docker run -d --name ipmon-server \
    --restart unless-stopped \
    psidex/ipmon-server:latest
```
