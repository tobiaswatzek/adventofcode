package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spitzfaust/adventofcode17/util"
)

func stringsToInts(fields []string) []int {
	var numbers []int
	for _, s := range fields {
		n, err := strconv.Atoi(s)
		util.Check(err)
		numbers = append(numbers, n)
	}
	return numbers
}

func findMaxIndex(numbers []int) int {
	maxIndex := 0
	max := numbers[maxIndex]
	for i, n := range numbers {
		if i == 0 {
			continue
		}
		if n > max {
			maxIndex = i
			max = n
		}
	}
	return maxIndex
}

func main() {
	f, err := os.Open("./input.txt")
	util.Check(err)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	if !scanner.Scan() {
		panic("Could not read line.")
	}
	line := scanner.Text()
	fields := strings.Fields(line)
	numbers := stringsToInts(fields)
	states := make(map[string]int)
	count := 0
	size := len(numbers)
	var currentState string
	for {
		currentState = fmt.Sprint(numbers)
		if _, found := states[currentState]; found {
			break
		} else {
			states[currentState] = count
		}

		maxIndex := findMaxIndex(numbers)
		toDistribute := numbers[maxIndex]
		numbers[maxIndex] = 0
		for i := (maxIndex + 1) % size; toDistribute > 0; i = (i + 1) % size {
			numbers[i]++
			toDistribute--
		}
		count++
	}
	cycles := count - states[currentState]
	fmt.Println(currentState)
	fmt.Print("Cycles: ")
	fmt.Println(cycles)
	fmt.Print("Count: ")
	fmt.Println(count)

}
