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
    |> Seq.map (fun (line) -> Regex.Replace(line, "[abcdefghijklmnopqrstuvwxyz]", ""))
    |> Seq.map (fun (digits) -> $"%c{digits[0]}%c{digits[digits.Length - 1]}")
    |> Seq.map int
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

let numberWords =
    [ "one", 1
      "two", 2
      "three", 3
      "four", 4
      "five", 5
      "six", 6
      "seven", 7
      "eight", 8
      "nine", 9 ]
    |> dict

let findNumAt (input: string) index =
    seq {
        for i in 1..9 do
            yield (i |> string, i)

        for KeyValue (name, value) in numberWords do
            yield (name, value)
    }
    |> Seq.choose (fun ((name, value)) ->
        let test = input.[index .. index + name.Length - 1]

        // printfn $"({name} {value}) %s{test}"

        if test = name then
            Some(value)
        else
            None)
    |> Seq.tryExactlyOne

let findNums (input: string) =
    [ for i in 0 .. input.Length do
          findNumAt input i ]
    |> Seq.choose id
    |> Seq.toArray

findNumAt "one2" 0
findNumAt "one2" 3

findNums "one2"
findNums "eighttwothree"
findNums "xtwoone3four"

let part2 (data: string) =
    data.Split("\n", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map findNums
    |> Seq.map (fun (digits) -> digits[0] * 10 + digits[digits.Length - 1])
    |> Seq.sum

part2 sample2 // 281
part2 input // 55260
