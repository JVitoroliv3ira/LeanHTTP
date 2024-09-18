package utils

import (
	"encoding/json"
	"fmt"
	"main/client"
)

func PrintResponse(res *client.HttpResponse) {
	fmt.Printf("%s %s (Time: %v)\n", res.Version, res.Status, res.Duration)

	fmt.Println("\nHeaders:")
	for key, values := range res.Headers {
		for _, value := range values {
			fmt.Printf("%s: %s\n", key, value)
		}
	}

	var prettyJSON map[string]interface{}
	err := json.Unmarshal(res.Body, &prettyJSON)
	if err != nil {
		fmt.Println("\nBody:")
		fmt.Println(string(res.Body))
	} else {
		prettyBody, _ := json.MarshalIndent(prettyJSON, "", "  ")
		fmt.Println("\nBody:")
		fmt.Println(string(prettyBody))
	}
}
