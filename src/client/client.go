package client

import (
	"io"
	"net/http"
	"time"
)

type HttpResponse struct {
	Status   string
	Version  string
	Headers  http.Header
	Body     []byte
	Duration time.Duration
}

func GetRequest(url string) (*HttpResponse, error) {
	client := &http.Client{Timeout: 30 * time.Second}

	start := time.Now()

	req, err := http.NewRequest("GET", url, nil)

	if err != nil {
		return nil, err
	}

	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}

	body, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}

	duration := time.Since(start)

	response := &HttpResponse{
		Status:   res.Status,
		Version:  res.Proto,
		Headers:  res.Header,
		Body:     body,
		Duration: duration,
	}

	return response, nil
}
