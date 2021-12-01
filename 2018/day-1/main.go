package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func getFinal(parts []string) int64 {
	var final int64 = 0

	for i := 0; i < len(parts); i++ {
		value, err := strconv.ParseInt(parts[i][1:], 10, 32)

		check(err)

		if parts[i][0] == '+' {
			final += value
		} else {
			final -= value
		}
	}

	return final
}

func getFirstRepeat(parts []string) int64 {
	var result int64 = 0
	var repeat *int64
	var seen = make(map[int64]int64)

	for repeat == nil {

		for i := 0; i < len(parts); i++ {
			curr := parts[i]
			value, err := strconv.ParseInt(curr[1:], 10, 32)

			check(err)

			if curr[0] == '+' {
				result += value
			} else {
				result -= value
			}

			if val, ok := seen[result]; ok && repeat == nil {
				return val
			}

			seen[result] = result
		}
	}

	return 0
}

func main() {
	fmt.Println("Starting...")

	dat, err := ioutil.ReadFile("input.txt")

	check(err)

	parts := strings.Split(string(dat), "\n")

	final := getFinal(parts)
	repeat := getFirstRepeat(parts)

	fmt.Println("Final Value:\t", final)
	fmt.Println("First Repeat:\t", repeat)
}
