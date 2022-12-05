#!/bin/bash

# Figure out the next day number
day=0

for dir in day-*; do
    num=$(echo $dir | cut -d'-' -f2)

    if [ $num -gt $day ]; then
        day=$num
    fi
done

day=$((day+1))

echo "The next day is $day"

# Make Project
mix new "day-$day" --app "day_$day" --module "Day$day"

cd day-$day

# Update the mix.exs file with a test alias
sed -i 's/deps: deps()/deps: deps(),\n      aliases: [test: ["test --no-start"]]/' mix.exs

# Fetch input
cookie=$(cat .session)

curl -A "Mozilla/5.0" -H "Cookie: session=$cookie" -o input "https://adventofcode.com/2022/day/${day}/input"