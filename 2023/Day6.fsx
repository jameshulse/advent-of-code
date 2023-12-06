#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
open FSharpPlus
open FSharp.Text.RegexProvider

let sample =
    """
Time:      7  15   30
Distance:  9  40  200
"""
// hold dist
// 0 -> 0
// 1 -> 6
// 2 -> 10 (2 * (7 - 2))
// 3 -> 12 (3 * (7 - 3))
// 7 ->

let input = getInput 2023 6

let parseLine line =
    line
    |> splitSpaces
    |> Array.skip 1
    |> Array.map int

type Race = { Time: int; Distance: int }

let waysToWin (race: Race) =
    seq {
        for hold in 1 .. (race.Time) do
            (hold, hold * (race.Time - hold))
    }
    |> Seq.skipWhile (fun (_, dist) -> dist <= race.Distance)
    |> Seq.takeWhile (fun (_, dist) -> dist > race.Distance)
    |> Seq.length

waysToWin { Time = 7; Distance = 9 }
waysToWin { Time = 15; Distance = 40 }
waysToWin { Time = 30; Distance = 200 }

let part1 data =
    let lines = splitByLine data

    let races =
        parseLine lines[0]
        |> Array.zip (parseLine lines[1])
        |> Array.map (fun parts ->
            { Time = snd parts
              Distance = fst parts })

    races
    |> Array.map waysToWin
    |> Array.fold (fun acc curr -> acc * curr) 1

part1 sample // 288?
part1 input

waysToWin { Time = 71530; Distance = 940200 }

let part2 data =
    // waysToWin { Time = 60808676u; Distance = 601116315591300u }
    0

part2 sample
part2 input
