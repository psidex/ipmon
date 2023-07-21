# ipmon

A super simple app and web server to monitor my LANs public IP and notify me of a renewal using a forwardable text.

## Client

Currently the Dockerfile and GitHub action are tailored for an ARM64 build (to run on my Raspberry Pi 4B).

```bash
cd ~
git clone https://github.com/psidex/ipmon.git
cd ipmon
docker build -t psidex/ipmon:latest .
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
