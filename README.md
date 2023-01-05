# ipmon

A super simple app to monitor my LANs public IP and notify me of a renewal.

Also sends a polite copyable text to send to work chat.

```bash
git clone https://github.com/psidex/ipmon.git
cd ipmon
docker build -t ipmon:latest .
docker run -d --name ipmon --restart unless-stopped ipmon:latest
```
