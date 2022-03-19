package ipcache

import (
	"io/ioutil"
	"os"

	"github.com/psidex/ipmon/internal/ip"
)

type IpCache struct {
	filePath string
}

func NewIpCache(file string) IpCache {
	return IpCache{file}
}

func (i IpCache) SetIp(toCache string) error {
	return ioutil.WriteFile(i.filePath, []byte(toCache), 0666)
}

func (i IpCache) GetIp() string {
	cached := ""

	if _, err := os.Stat(i.filePath); err == nil {
		data, err := ioutil.ReadFile(i.filePath)
		if err != nil {
			cached = ""
		}
		cached = string(data)
		if !ip.ValidAddress(cached) {
			cached = ""
		}
	}

	return cached
}
