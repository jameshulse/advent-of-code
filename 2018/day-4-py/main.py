from collections import defaultdict
import re

def get_sleep_patterns(entries) -> dict:
    guard_track = defaultdict(lambda: defaultdict(int))

    started_sleep = None
    current_guard = None

    for entry in entries:
        minute = int(re.findall(':(\d{2})', entry)[0])

        if 'begins shift' in entry:
            current_guard = re.findall('#(\d+)', entry)[0]
        elif 'falls asleep' in entry:
            started_sleep = minute
        elif 'wakes up' in entry:
            for m in range(started_sleep, minute):
                guard_track[current_guard][m] += 1
    
    return guard_track


def main():
    print('Starting...')

    entries = sorted(open('input.txt').readlines())

    sleep = get_sleep_patterns(entries)

    print('Part One:')

    longest_sleeper = max(sleep, key=lambda k: sum(sleep[k].values()))
    most_asleep_minute = max(sleep[longest_sleeper], key=sleep[longest_sleeper].get)

    print('Longest sleeping guard: #', longest_sleeper)
    print('Guard #', longest_sleeper, 'slept most in', most_asleep_minute)
    print('Answer:', int(longest_sleeper) * most_asleep_minute)

    print('Part Two:')

    consistent_sleeper = max(sleep, key=lambda k: max(sleep[k].values()))
    most_asleep_minute = max(sleep[consistent_sleeper], key=sleep[consistent_sleeper].get)

    print('Most consistent Guard #', consistent_sleeper)
    print('Guard #', consistent_sleeper, 'slept most in', most_asleep_minute)
    print('Answer:', int(consistent_sleeper) * int(most_asleep_minute))

    print('Done')

if __name__ == "__main__":
    main()