package main

import "{{NAME}}/internal/router"

func main() {
	r := router.SetupRouter()
	r.Run(":8000")
}
