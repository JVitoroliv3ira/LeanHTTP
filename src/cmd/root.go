package cmd

import (
	"github.com/spf13/cobra"
	"main/version"
)

var rootCmd = cobra.Command{
	Use:     "lean_http",
	Short:   "A lightweight HTTP client for quick requests",
	Long:    "Lean HTTP is a command-line client for making quick and efficient HTTP requests.",
	Version: version.Version,
}

func Execute() error {
	return rootCmd.Execute()
}
