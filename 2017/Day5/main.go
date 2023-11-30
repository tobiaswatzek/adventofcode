package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spitzfaust/adventofcode17/util"
)

func stringsToNumbers(strings []string) []int {
	var numbers []int
	for _, s := range strings {
		number, err := strconv.Atoi(s)
		util.Check(err)
		numbers = append(numbers, number)
	}
	return numbers
}

func calculateNormalJumps(jumps []int) int {
	size := len(jumps)
	var count int
	for i := 0; i < size; {
		count++
		j := i
		i += jumps[i]
		jumps[j]++
	}
	return count
}

func calculateStrangeJumps(jumps []int) int {
	size := len(jumps)
	var count int
	for i := 0; i < size; {
		count++
		j := i
		i += jumps[i]
		if jumps[j] >= 3 {
			jumps[j]--
		} else {
			jumps[j]++
		}
	}
	return count
}

func main() {
	file, err := os.Open("./input.txt")
	util.Check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var jumps []int
	for scanner.Scan() {
		s := strings.TrimSpace(scanner.Text())
		if s != "" {
			jump, err := strconv.Atoi(s)
			util.Check(err)
			jumps = append(jumps, jump)
		}
	}

	fmt.Println(calculateNormalJumps(append([]int(nil), jumps...)))
	fmt.Println(calculateStrangeJumps(append([]int(nil), jumps...)))
}
