package ip

import (
	"io/ioutil"
	"net"
	"net/http"
)

func ValidAddress(ip string) bool {
	return net.ParseIP(ip) != nil
}

func GetPublicIp() string {
	resp, err := http.Get("https://api.ipify.org")
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}

	ip := string(body)

	if !ValidAddress(ip) {
		panic("Invalid IP address from ipify.org: " + ip)
	}

	return ip
}
