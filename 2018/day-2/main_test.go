package main

import "testing"

func TestGetStringIntersection(t *testing.T) {
	result := getStringIntersection("fghij", "fguij")

	if result != "fgij" {
		t.Errorf("Incorrect result, got: '%s', expected: 'fgij'", result)
	}
}

func TestGetStringDifferences_allDifferent(t *testing.T) {
	result := getStringDifferences("abcde", "fghij")

	if result != 5 {
		t.Errorf("Incorrect result, got: '%d', expected: 5", result)
	}
}

func TestGetStringDifferences_oneDifference(t *testing.T) {
	result := getStringDifferences("fguij", "fghij")

	if result != 1 {
		t.Errorf("Incorrect result, got: '%d', expected: 1", result)
	}
}

func TestGetStringDifferences_same(t *testing.T) {
	result := getStringDifferences("abcde", "abcde")

	if result != 0 {
		t.Errorf("Incorrect result, got: '%d', expected: 0", result)
	}
}
