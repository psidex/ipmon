package ip

import (
	"errors"
	"io/ioutil"
	"net"
	"net/http"
)

func ValidAddress(ip string) bool {
	return net.ParseIP(ip) != nil
}

func GetPublicIp() (string, error) {
	resp, err := http.Get("https://api.ipify.org")
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	ip := string(body)

	if !ValidAddress(ip) {
		return "", errors.New("Invalid IP address from ipify.org: " + ip)
	}

	return ip, nil
}
