package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math/rand"
	"os"
	"time"

	"github.com/psidex/ipmon/internal/ip"
	"github.com/psidex/ipmon/internal/twilio"
)

const cacheFile = "./ipmon.cache"

func cacheIp(toCache string) {
	err := ioutil.WriteFile(cacheFile, []byte(toCache), 0666)
	if err != nil {
		panic(err)
	}
}

func ipFromCache() string {
	cached := ""

	if _, err := os.Stat(cacheFile); err == nil {
		data, err := ioutil.ReadFile(cacheFile)
		if err != nil {
			panic(err)
		}
		cached = string(data)
		if !ip.ValidAddress(cached) {
			cached = ""
		}
	}

	return cached
}

func main() {
	twilioSid := os.Getenv("TWILIO_SID")
	twilioToken := os.Getenv("TWILIO_TOKEN")
	twilioFrom := os.Getenv("IPMON_TWILIO_FROM")
	to := os.Getenv("IPMON_TO")

	if twilioSid == "" || twilioToken == "" || twilioFrom == "" || to == "" {
		panic("TWILIO_SID, TWILIO_TOKEN, IPMON_TWILIO_FROM, IPMON_TO must be set")
	}

	rand.Seed(time.Now().Unix())

	requests := []string{
		"Could I get a new IP please? %s",
		"Can I have a new IP? %s",
		"Could someone update my IP please %s",
		"I need a new IP please, %s",
	}

	prevIp := ipFromCache()
	log.Printf("Started, loaded IP from cache: %s", prevIp)

	t := twilio.NewTwilio(twilioSid, twilioToken, twilioFrom)

	for {
		currentIp := ip.GetPublicIp()

		if currentIp != prevIp {
			log.Printf("IP Changed from %s to %s", prevIp, currentIp)

			t.SendText(to, fmt.Sprintf("IP Changed from %s to %s", prevIp, currentIp))
			t.SendText(to, fmt.Sprintf(requests[rand.Intn(len(requests))], currentIp))

			cacheIp(currentIp)
			prevIp = currentIp
		}

		time.Sleep(time.Minute * 1)
	}
}
