package main

import (
	"fmt"
)

func PrintError(err error) {
	error_str := fmt.Errorf("Debug Error: %v", err)
	if error_str != nil {
		fmt.Println(error_str)
	}
}

func InfoLog(msg string) {
	fmt.Println("Info:", msg)
}
