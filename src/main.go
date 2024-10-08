package main

import (
	"log"
	"main/cmd"
)

func main() {
	if err := cmd.Execute(); err != nil {
		log.Fatalf("Error executing command: %v", err)
	}
}
