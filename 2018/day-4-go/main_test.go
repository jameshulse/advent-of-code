package main

import "testing"

func TestGetResult(t *testing.T) {
	entries := []string{
		"[1518-11-01 00:00] Guard #10 begins shift",
		"[1518-11-01 00:05] falls asleep",
		"[1518-11-01 00:25] wakes up",
		"[1518-11-01 00:30] falls asleep",
		"[1518-11-01 00:55] wakes up",
		"[1518-11-01 23:58] Guard #99 begins shift",
		"[1518-11-02 00:40] falls asleep",
		"[1518-11-02 00:50] wakes up",
		"[1518-11-03 00:05] Guard #10 begins shift",
		"[1518-11-03 00:24] falls asleep",
		"[1518-11-03 00:29] wakes up",
		"[1518-11-04 00:02] Guard #99 begins shift",
		"[1518-11-04 00:36] falls asleep",
		"[1518-11-04 00:46] wakes up",
		"[1518-11-05 00:03] Guard #99 begins shift",
		"[1518-11-05 00:45] falls asleep",
		"[1518-11-05 00:55] wakes up",
	}

	sleep_map := GetSleepMap(entries)
	guard, total_sleep := GetMaxFromMap(sleep_map)

	if guard != 10 {
		t.Errorf("Incorrect guard, got: %v, expected: 10", guard)
	}

	if total_sleep != 24 {
		t.Errorf("Incorrect sleep value, got: %v, expected 24", total_sleep)
	}
}
