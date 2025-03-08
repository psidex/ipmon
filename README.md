# ipmon

ipmon is a CLI tool which can monitor a machines public IP by periodically requesting it from a web server on the internet, such as [checkip.amazonaws.com](https://checkip.amazonaws.com/). If an IP change is detected, it can send notifications via most services by using [apprise](https://github.com/caronc/apprise).

## Deploy Using Docker

```bash
docker run -d --name ipmon \
    --restart unless-stopped \
    -v $(pwd)/config.yaml:/config.yaml \
    psidex/ipmon:latest
```

## Config

ipmon is configured using a `config.yaml` file, this should exist in the working directory, or the path can be provided as an arg, e.g. `ipmon ./path/to/config.yaml`.

If you're running the Docker image, mount your config file to `/config.yaml` inside the container. If you want access to the IP cache file, mount a file to `/ipmon.cache`.

Use the `server` key to set the IP web server, this can be any URL that returns a **plaintext IPv4**, IPv6 is unsupported.

The `interval` key is the number of seconds ipmon will wait between checking for an IP change.

If the `notifications` key is set, it should contain an array of objects, each with a `title`, `body`, and `url`. `body` can contain the string `{{ip}}` which will get replaced with the newly detected IP. These will be passed into [apprise](https://github.com/caronc/apprise), which **will need to be installed and in your PATH if you're using ipmon as a standalone binary** (don't worry about this if you're using the Docker image).

### Example config.yaml

```yaml
server: "https://checkip.amazonaws.com/"
interval: 600
notifications:
  - title: "ipmon"
    body: "IP changed to {{ip}}"
    url: "twilio://abc:def@123/456"
```
