# ipmon

A super simple app to monitor my LANs public IP and notify me of a renewal using a forwardable text.

Currently the Dockerfile and GitHub action are tailored for an ARM64 build (to run on my Raspberry Pi 4B).

```bash
git clone https://github.com/psidex/ipmon.git
cd ipmon
vi ipmon.env
docker build -t psidex/ipmon:latest .
docker run -d --name ipmon \
    --restart unless-stopped \
    --env-file $(pwd)/ipmon.env \
    psidex/ipmon:latest
```
