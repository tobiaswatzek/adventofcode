package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"

	"github.com/spitzfaust/adventofcode17/util"
)

func uniqueFields(fields []string) bool {
	for i, s := range fields {
		for j, f := range fields {
			if i != j && s == f {
				return false
			}
		}
	}
	return true
}

func sortString(w string) string {
	s := strings.Split(w, "")
	sort.Strings(s)
	return strings.Join(s, "")
}

func containsNoAnagrams(fields []string) bool {
	for i, s := range fields {
		sSorted := sortString(s)
		for j, f := range fields {
			if i == j {
				continue
			}
			fSorted := sortString(f)
			if sSorted == fSorted {
				return false
			}
		}
	}
	return true
}

func main() {
	file, err := os.Open("./input.txt")
	util.Check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var validUniquePhrases int
	var validNoAnagramPhrases int

	for scanner.Scan() {
		row := scanner.Text()
		fields := strings.Fields(row)
		if uniqueFields(fields) {
			validUniquePhrases++
		}
		if containsNoAnagrams(fields) {
			validNoAnagramPhrases++
		}

	}
	fmt.Print("Valid Unique: ")
	fmt.Println(validUniquePhrases)
	fmt.Print("Valid No Anagram: ")
	fmt.Println(validNoAnagramPhrases)
}
