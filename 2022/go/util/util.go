package util

import (
	"log"
	"os"
)

func ReadFile(filepath string) string {
	data, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatal(err)
	}

	return string(data)
}

func Sum(numbers []int) int {
	var sum int
	for _, number := range numbers {
		sum += number
	}
	return sum
}
