package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	// Prompt user for input
	reader := bufio.NewReader(os.Stdin)

	fmt.Print("Enter password: ")
	password, _ := reader.ReadString('\n')

	fmt.Print("Enter universal path: ")
	universalPath, _ := reader.ReadString('\n')

	fmt.Print("Enter db settings URL: ")
	dbSettingsURL, _ := reader.ReadString('\n')

	// Create TOML file
	fileContent := fmt.Sprintf(`[server_settings]
password = "%s"
universal_path = "%s"
[db_settings]
db_settings_url = "%s"
`, password, universalPath, dbSettingsURL)

	err := os.WriteFile("env.toml", []byte(fileContent), 0644)
	if err != nil {
		fmt.Println("Error creating TOML file:", err)
		return
	}

	fmt.Println("TOML file created successfully!")
}

