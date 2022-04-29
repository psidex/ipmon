package stringcache

import (
	"io/ioutil"
	"os"
)

// Cache stores a single string on disk.
type Cache struct {
	filePath string
}

func NewCache(file string) Cache {
	return Cache{file}
}

func (i Cache) Set(str string) error {
	return ioutil.WriteFile(i.filePath, []byte(str), 0666)
}

func (i Cache) Get() string {
	if _, err := os.Stat(i.filePath); err == nil {
		data, err := ioutil.ReadFile(i.filePath)
		if err != nil {
			return ""
		}
		return string(data)
	}
	return ""
}
