#r "nuget: FsHttp"
#load "Advent.fs"

open System
open System.Text.RegularExpressions
open Advent

let sample =
    """
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""

let input = getInput 2023 1

let part1 (data: string) =
    data.Split("\n", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map (fun (line) -> Regex.Replace(line, @"[^\d]", ""))
    |> Seq.map (fun (digits) -> int $"%c{digits[0]}%c{digits[digits.Length - 1]}")
    |> Seq.sum

part1 sample // 142
part1 input // 55123

let sample2 =
    """
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""

let matchToNumber (input: Match) =
    match input.Value with
    | "one" -> 1
    | "two" -> 2
    | "three" -> 3
    | "four" -> 4
    | "five" -> 5
    | "six" -> 6
    | "seven" -> 7
    | "eight" -> 8
    | "nine" -> 9
    | v -> int v

let getCalibration (input: string) =
    let pattern = @"\d|one|two|three|four|five|six|seven|eight|nine"
    let first = Regex(pattern).Match(input)

    let last =
        Regex(pattern, RegexOptions.RightToLeft)
            .Match(input)

    (matchToNumber first) * 10 + (matchToNumber last)

let part2Take2 (data: string) =
    data.Split("\n", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.sumBy getCalibration

part2Take2 sample2 // 281
part2Take2 input // 55260
