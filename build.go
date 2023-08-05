package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

const (
	SERVER_DIR_NAME      = "server"
	CONFIG_CLIENT_DIR_NAME = "config-client"
	BUILD_DIR_NAME       = "build"
)

func main() {
	// Get the current directory
	mydir, err := os.Getwd()
	if err != nil {
		fmt.Println("Error getting current directory:", err)
		return
	}

	// Define directory paths
	serverDir := filepath.Join(mydir, SERVER_DIR_NAME)
	configClientDir := filepath.Join(mydir, CONFIG_CLIENT_DIR_NAME)
	buildDir := filepath.Join(mydir, BUILD_DIR_NAME)

	// Delete build directory
	if _, err := os.Stat(buildDir); err == nil {
		err := os.RemoveAll(buildDir)
		if err != nil {
			fmt.Println("Error deleting build directory:", err)
			return
		}
	}

	// Build server and config client
	serverBuildCmd := exec.Command("cargo", "build", "--manifest-path="+filepath.Join(serverDir, "Cargo.toml"))
	serverBuildCmd.Stdout = os.Stdout
	serverBuildCmd.Stderr = os.Stderr
	err = serverBuildCmd.Run()
	if err != nil {
		fmt.Println("Error building server:", err)
		return
	}

	configClientBuildCmd := exec.Command("yarn", "run", "build")
	configClientBuildCmd.Dir = configClientDir
	configClientBuildCmd.Stdout = os.Stdout
	configClientBuildCmd.Stderr = os.Stderr
	err = configClientBuildCmd.Run()
	if err != nil {
		fmt.Println("Error building config client:", err)
		return
	}

	// Move files to build directory
	moveCmd := exec.Command("mv", filepath.Join(serverDir, "target"), buildDir)
	err = moveCmd.Run()
	if err != nil {
		fmt.Println("Error moving server files:", err)
		return
	}

	moveCmd = exec.Command("mv", filepath.Join(configClientDir, "build"), filepath.Join(buildDir, "static"))
	err = moveCmd.Run()
	if err != nil {
		fmt.Println("Error moving config client files:", err)
		return
	}

	fmt.Println("Build completed successfully.")
}

