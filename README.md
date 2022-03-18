# ipmon

A super simple app to monitor my LANs public IP and notify me of a renewal.

Also sends a polite copyable text to send to work chat.

## Setup & Run

```bash
cd ipmon
chmod +x *.sh
./build.sh
cp ipmon.service.example ipmon.service
vi ipmon.service # Edit to your config
./initservice.sh # Takes a few seconds, should show green status
```
