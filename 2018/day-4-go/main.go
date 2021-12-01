package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func GetSleepMap(entries []string) map[int]map[int]int {
	sort.Strings(entries)

	sleep_track := make(map[int]map[int]int)

	var current_guard int
	var sleep_minute int

	guard_pat := regexp.MustCompile(`#(\d+)`)
	minute_pat := regexp.MustCompile(`:(\d{2})`)

	for _, entry := range entries {
		current_minute, _ := strconv.Atoi(minute_pat.FindString(entry)[1:])

		if strings.Contains(entry, "begins shift") {
			guard, _ := strconv.Atoi(guard_pat.FindString(entry)[1:])

			current_guard = guard
		} else if strings.Contains(entry, "falls asleep") {
			sleep_minute = current_minute
		} else if strings.Contains(entry, "wakes up") {

			for i := sleep_minute; i < current_minute; i++ {
				sleep_track[current_guard] = append(sleep_track[current_guard], i)
			}
		}
	}

	return sleep_track
}

func GetMaxFromMap(target map[int][]int) (int, int) {
	var max_key int = 0
	var max_value int = 0

	// for k, v := range target {
	// 	if v > max_value {
	// 		max_key = k
	// 		max_value = v
	// 	}
	// }

	return max_key, max_value
}

func main() {
	dat, err := ioutil.ReadFile("input.txt")

	check(err)

	entries := strings.Split(string(dat), "\n")

	sleep_track := GetSleepMap(entries)
	guard, total_sleep := GetMaxFromMap(sleep_track)

	fmt.Println("Longest sleeping guard:", guard)
	fmt.Println("Longest sleep (minutes):", total_sleep)
	fmt.Println("ID:", guard*total_sleep)
}
