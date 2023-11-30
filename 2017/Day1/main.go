package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/spitzfaust/adventofcode17/util"
)

// StringToNumbers converts a string to a slice of ints.
func StringToNumbers(s string) []int {
	var numbers []int
	s = strings.TrimSpace(s)
	for _, r := range s {
		c := string(r)
		number, err := strconv.Atoi(c)
		util.Check(err)
		numbers = append(numbers, number)
	}
	return numbers
}

// FindNumbersForSum finds numbers for sum.
func FindNumbersForSum(numbers []int) []int {
	size := len(numbers)
	var numbersForSum []int
	for i, j := 0, 1; j < size; i, j = i+1, j+1 {
		if numbers[i] == numbers[j] {
			numbersForSum = append(numbersForSum, numbers[i])
		}
	}
	if numbers[0] == numbers[size-1] {
		numbersForSum = append(numbersForSum, numbers[0])
	}
	return numbersForSum
}

// FindNumbersForHalfwaySum finds the numbers for the halfway sum.
func FindNumbersForHalfwaySum(numbers []int) []int {
	size := len(numbers)
	middle := size / 2
	var numbersForSum []int
	for i := 0; i < size; i++ {
		j := (middle + i) % size
		if numbers[i] == numbers[j] {
			numbersForSum = append(numbersForSum, numbers[i])
		}
	}
	return numbersForSum
}

func main() {
	s := util.ReadFileToString("./input.txt")
	fmt.Println(s)
	numbers := StringToNumbers(s)

	numbersForSum := FindNumbersForSum(numbers)
	sum := util.Sum(numbersForSum)
	fmt.Println(sum)

	numbersForHalfwaySum := FindNumbersForHalfwaySum(numbers)
	halfwaySum := util.Sum(numbersForHalfwaySum)
	fmt.Println(halfwaySum)
}
