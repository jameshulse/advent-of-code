package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func calculateChecksum(codes []string) int {
	twos := 0
	threes := 0

	for i := 0; i < len(codes); i++ {
		occurs := make(map[rune]int)

		code := codes[i]

		for _, alpha := range code {
			occurs[alpha] += 1
		}

		count_two := 0
		count_three := 0

		for key := range occurs {
			if occurs[key] == 2 {
				count_two = 1
			} else if occurs[key] == 3 {
				count_three = 1
			}
		}

		twos += count_two
		threes += count_three
	}

	return twos * threes
}

func findCodesWithSingleDifference(codes []string) (string, string) {
	for _, codeA := range codes {
		for _, codeB := range codes {
			if getStringDifferences(codeA, codeB) == 1 {
				return codeA, codeB
			}
		}
	}

	return "", "" // TODO: This isn't right..
}

func getStringDifferences(left string, right string) int {
	result := 0

	for i := 0; i < len(left); i++ {
		if left[i] != right[i] {
			result += 1
		}
	}

	return result
}

func getStringIntersection(left string, right string) string {
	result := ""

	for i := 0; i < len(left); i++ {
		if left[i] == right[i] {
			result += string(left[i])
		}
	}

	return result
}

func main() {
	dat, err := ioutil.ReadFile("input.txt")

	check(err)

	codes := strings.Split(string(dat), "\n")

	checksum := calculateChecksum(codes)

	similarA, similarB := findCodesWithSingleDifference(codes)
	common := getStringIntersection(similarA, similarB)

	fmt.Println("Checksum:\t", checksum)
	fmt.Println("In Common:\t", common)
}
