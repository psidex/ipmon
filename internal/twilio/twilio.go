package twilio

import (
	"net/http"
	"net/url"
	"strings"
)

type Twilio struct {
	sid   string
	token string
	from  string
	url   string
}

func NewTwilio(sid, token, from string) Twilio {
	return Twilio{
		sid,
		token,
		from,
		"https://api.twilio.com/2010-04-01/Accounts/" + sid + "/Messages.json",
	}
}

func (t Twilio) SendText(to, msg string) error {
	v := url.Values{}
	v.Set("To", to)
	v.Set("From", t.from)
	v.Set("Body", msg)

	rb := strings.NewReader(v.Encode())
	req, err := http.NewRequest("POST", t.url, rb)
	if err != nil {
		return err
	}

	req.SetBasicAuth(t.sid, t.token)
	req.Header.Add("Accept", "application/json")
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")

	client := &http.Client{}
	_, err = client.Do(req)
	return err
}
