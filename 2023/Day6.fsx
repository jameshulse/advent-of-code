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
// x = h * (t - h)
// h^2 - ht + x = 0

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
    |> Array.map uint64

type Race = { Time: uint64; Distance: uint64 }

let waysToWin (race: Race) =
    seq {
        let mutable hold = 1UL

        while true do
            yield hold
            hold <- hold + 1UL
    }
    |> Seq.map (fun (hold) -> (hold, hold * (race.Time - hold)))
    |> Seq.skipWhile (fun (_, dist) -> dist <= race.Distance)
    |> Seq.takeWhile (fun (hold, dist) -> dist > race.Distance && hold < race.Time)
    |> Seq.length

waysToWin { Time = 7UL; Distance = 9UL } // 4
waysToWin { Time = 15UL; Distance = 40UL } // 8
waysToWin { Time = 30UL; Distance = 200UL } // 9

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

part1 sample // 288
part1 input // 1155175

waysToWin { Time = 71530UL; Distance = 940200UL } // 71503

let part2 =
    let time = 60_80_86_76UL
    let distance = 601_1163_1559_1300UL

    (* Brute force! *)
    waysToWin { Time = time; Distance = distance }

part2 // 35961505
