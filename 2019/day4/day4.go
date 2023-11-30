package main

import (
	"fmt"
	"strconv"

	"github.com/spitzfaust/adventofcode2019/util"
)

func hasDouble(str string) bool {
	for index := 1; index < len(str); index++ {
		if str[index] == str[index-1] {
			return true
		}
	}
	return false
}

func isIncreasing(str string) bool {
	for index := 1; index < len(str); index++ {
		a, err := strconv.Atoi(string(str[index-1]))
		util.Check(err)
		b, err := strconv.Atoi(string(str[index]))
		util.Check(err)
		if a > b {
			return false
		}
	}

	return true
}

func main() {
	lower := 145852
	upper := 616942

	numberOfPasswords := 0

	for index := lower; index < upper; index++ {
		str := strconv.Itoa(index)
		if hasDouble(str) && isIncreasing(str) {
			numberOfPasswords++
		}
	}

	fmt.Println(numberOfPasswords)
}
