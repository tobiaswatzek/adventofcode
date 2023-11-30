package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/spitzfaust/adventofcode17/util"
)

func calculateMinMaxChecksum(fields []string) int {
	max, err := strconv.Atoi(fields[0])
	util.Check(err)
	min, err := strconv.Atoi(fields[0])
	util.Check(err)
	for _, s := range fields[1:] {
		x, err := strconv.Atoi(s)
		util.Check(err)
		if x < min {
			min = x
		}
		if x > max {
			max = x
		}
	}
	return max - min
}

func calculateDivisionChecksum(fields []string) int {
	var checksum int
search:
	for i, s := range fields {
		x, err := strconv.Atoi(s)
		util.Check(err)
		for j, f := range fields {
			if i == j {
				continue
			}
			y, err := strconv.Atoi(f)
			util.Check(err)
			if (x % y) == 0 {
				checksum = x / y
				break search
			}
		}
	}
	return checksum
}

func main() {
	file, err := os.Open("./input.txt")
	util.Check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var minMaxChecksums []int
	var divisionChecksums []int

	for scanner.Scan() {
		row := scanner.Text()
		fields := strings.Fields(row)
		minMaxChecksums = append(minMaxChecksums, calculateMinMaxChecksum(fields))
		divisionChecksums = append(divisionChecksums, calculateDivisionChecksum(fields))
	}
	minMaxChecksum := util.Sum(minMaxChecksums)
	fmt.Println(minMaxChecksum)
	divisionChecksum := util.Sum(divisionChecksums)
	fmt.Println(divisionChecksum)
}
