package main

import (
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"

	"github.com/psidex/ipmon/internal/ip"
	"github.com/psidex/ipmon/internal/ipcache"
	"github.com/psidex/ipmon/internal/twilio"
)

const cacheFile = "./ipmon.cache"

func main() {
	twilioSid := os.Getenv("TWILIO_SID")
	twilioToken := os.Getenv("TWILIO_TOKEN")
	twilioFrom := os.Getenv("IPMON_TWILIO_FROM")
	to := os.Getenv("IPMON_TO")

	if twilioSid == "" || twilioToken == "" || twilioFrom == "" || to == "" {
		panic("TWILIO_SID, TWILIO_TOKEN, IPMON_TWILIO_FROM, IPMON_TO must be set")
	}

	requests := []string{
		"Could I get a new IP please? %s",
		"Can I have a new IP? %s",
		"Could someone update my IP please %s",
		"I need a new IP please, %s",
	}

	rand.Seed(time.Now().Unix())

	t := twilio.NewTwilio(twilioSid, twilioToken, twilioFrom)
	c := ipcache.NewIpCache(cacheFile)

	prevIp := c.GetIp()
	log.Printf("Started, loaded IP from cache: %s", prevIp)

	for {
		time.Sleep(time.Minute * 1)

		currentIp, err := ip.GetPublicIp()
		if err != nil {
			log.Printf("Error getting public IP: %s", err)
			continue
		}

		if currentIp != prevIp {
			log.Printf("IP Changed from %s to %s", prevIp, currentIp)

			err = t.SendText(to, fmt.Sprintf("IP Changed from %s to %s", prevIp, currentIp))
			if err != nil {
				log.Printf("Error sending text: %s", err)
			}

			err = t.SendText(to, fmt.Sprintf(requests[rand.Intn(len(requests))], currentIp))
			if err != nil {
				log.Printf("Error sending text: %s", err)
			}

			err = c.SetIp(currentIp)
			if err != nil {
				log.Printf("Error setting ip cache: %s", err)
			}

			prevIp = currentIp
		}
	}
}
