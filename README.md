# ipmon

A super simple app to monitor my LANs public IP and notify me of a renewal.

Also sends a polite copyable text to send to work chat.

## Setup & Run

```bash
cd ipmon
chmod +x *.sh
cp ipmon.service.example ipmon.service
vi ipmon.service # Edit to your config
./deploy.sh # Takes a few seconds, should show green status
```

Run `teardown.sh` to stop and remove the service.
