package main

import (
	"os"
	"strings"

	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/plugins/migratecmd"
)

func main() {
	// Check if the program is in development mode.
	isGoRun := strings.HasPrefix(os.Args[0], os.TempDir())

	// PocketBase app instance.
	app := pocketbase.New()

	// Register the migration command.
	migratecmd.MustRegister(app, app.RootCmd, migratecmd.Config{
		Automigrate: isGoRun,
	})

	if err := app.Start(); err != nil {
		app.Logger().Error("Failed to start the app: %v", err)
	}
}