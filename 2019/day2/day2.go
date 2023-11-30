package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/spitzfaust/adventofcode2019/util"
)

func lineToIntSlice(line string) []int {
	parts := strings.Split(line, ",")
	ints := make([]int, len(parts))
	for index, part := range parts {
		val, err := strconv.Atoi(part)
		util.Check(err)
		ints[index] = val
	}
	return ints
}

func processInstruction(index int, ints []int) error {
	type operandType int
	const (
		add      operandType = 1
		multiply operandType = 2
		halt     operandType = 99
	)

	type instruction struct {
		operand       operandType
		firstAddress  int
		secondAddress int
		toAddress     int
	}

	inst := instruction{
		operand:       operandType(ints[index+0]),
		firstAddress:  ints[index+1],
		secondAddress: ints[index+2],
		toAddress:     ints[index+3],
	}

	switch inst.operand {
	case add:
		ints[inst.toAddress] = ints[inst.firstAddress] + ints[inst.secondAddress]
	case multiply:
		ints[inst.toAddress] = ints[inst.firstAddress] * ints[inst.secondAddress]
	case halt:
		log.Println("Operand halt encountered.")
		return errors.New("halt")
	default:
		log.Printf("Unexpected operand %d", inst.operand)
		return errors.New("unexpected operand")
	}
	return nil
}

func runIntCode(ints []int) int {

	ints[1] = 12
	ints[2] = 2

	for index := 0; index < len(ints); index += 4 {
		err := processInstruction(index, ints)
		if err == nil {
			continue
		}
		if err.Error() == "halt" {
			break
		}
		panic(err)
	}
	return ints[0]
}

func main() {
	file, err := os.Open("./input.txt")
	util.Check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	util.Check(err)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	ints := lineToIntSlice(line)
	fmt.Println(runIntCode(ints))
}
