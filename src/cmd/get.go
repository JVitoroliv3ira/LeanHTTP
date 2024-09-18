package cmd

import (
	"github.com/spf13/cobra"
	"log"
	"main/client"
	"main/utils"
)

func init() {
	rootCmd.AddCommand(getCommand)
}

var getCommand = &cobra.Command{
	Use:     "get [url]",
	Short:   "Makes an HTTP GET request to the specified URL",
	Long:    "The 'get' command makes an HTTP GET request to the provided URL.\nYou must provide a valid URL as the first argument.",
	Args:    cobra.MinimumNArgs(1),
	Example: "lean_http get http://localhost:9000\nlean_http get https://example.com",
	Run: func(cmd *cobra.Command, args []string) {
		url := args[0]

		res, err := client.GetRequest(url)
		if err != nil {
			log.Fatalf("Error making request: %v", err)
		}

		utils.PrintResponse(res)
	},
}
