package stringcache

import (
	"io/ioutil"
)

// Cache stores a single string on disk.
type Cache struct {
	filePath string
}

func NewCache(filePath string) Cache {
	return Cache{filePath}
}

func (c Cache) Set(str string) error {
	return ioutil.WriteFile(c.filePath, []byte(str), 0666)
}

func (c Cache) Get() string {
	data, err := ioutil.ReadFile(c.filePath)
	if err != nil {
		return ""
	}
	return string(data)
}
