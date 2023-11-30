package main

import (
	"adventofcode2022/days"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
)

type arguments struct {
	dayNumber int
	dataDir   string
}

func main() {
	fmt.Println("AoC 2022")
	args := parseArguments()
	inputFilePath := inputFilePathForDay(args)
	solutionPartOne, solutionPartTwo, err := days.RunDay(days.RunDayData{Number: args.dayNumber, InputFilePath: inputFilePath})
	if err != nil {
		panic(err)
	}
	fmt.Printf("Day %d\n\tPart One: %s\n\tPart Two:%s\n", args.dayNumber, solutionPartOne, solutionPartTwo)
}

func inputFilePathForDay(args arguments) string {
	return filepath.Join(args.dataDir, fmt.Sprintf("day%d.txt", args.dayNumber))
}

func parseArguments() arguments {
	if len(os.Args[1:]) != 2 {
		panic("Expecting exactly two arguments.")
	}

	dayNumber, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic(err)
	}

	if dayNumber < 1 || dayNumber > 24 {
		panic("Only accepting numbers between 1 and 24")
	}

	dataDir := os.Args[2]

	return arguments{dayNumber: dayNumber, dataDir: dataDir}
}
