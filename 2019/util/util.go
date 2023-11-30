package util

import "io/ioutil"

import "log"

// Check panics if the given error is not nil.
func Check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}

// ReadFileToString reads the contents of a file into a string.
func ReadFileToString(filepath string) string {
	dat, err := ioutil.ReadFile(filepath)
	Check(err)
	return string(dat)
}

// Sum returns the sum of an int slice.
func Sum(numbers []int) int {
	var sum int
	for _, number := range numbers {
		sum += number
	}
	return sum
}
