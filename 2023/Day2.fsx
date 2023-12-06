#r "nuget: FsHttp"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
open FSharp.Text.RegexProvider

let sample =
    """
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""

let input = getInput 2023 2

type Game =
    { Id: int
      Red: int
      Green: int
      Blue: int }

type ParseReveal = Regex< @"(?<Count>\d+) (?<Colour>\w+)" >

let (|Red|Green|Blue|) reveal =
    let ballSearch = ParseReveal().TypedMatch(reveal)
    let count = int ballSearch.Count.Value

    match ballSearch.Colour.Value with
    | "red" -> Red(count)
    | "green" -> Green(count)
    | "blue" -> Blue(count)
    | _ -> failwithf $"Unknown colour: {ballSearch.Colour}"

type ParseGameId = Regex< @"Game (?<Id>\d+):" >

let parseGame line =
    let id = int (ParseGameId().TypedMatch(line).Id.Value)
    let rounds = line.Substring(line.IndexOf(": ") + 2).Split("; ")

    let mutable maxRed = 0
    let mutable maxGreen = 0
    let mutable maxBlue = 0

    for round in rounds do
        for reveal in round.Split(", ") do
            match reveal with
            | Red (count) -> maxRed <- Math.Max(count, maxRed)
            | Green (count) -> maxGreen <- Math.Max(count, maxGreen)
            | Blue (count) -> maxBlue <- Math.Max(count, maxBlue)

    { Id = id
      Red = maxRed
      Green = maxGreen
      Blue = maxBlue }

let gameMeetsThreshold (maxRed, maxGreen, maxBlue) game =
    game.Red <= maxRed
    && game.Green <= maxGreen
    && game.Blue <= maxBlue

let part1 data =
    let meetsElfThreshold = gameMeetsThreshold (12, 13, 14)

    splitByLine data
    |> Seq.map parseGame
    |> Seq.filter meetsElfThreshold
    |> Seq.sumBy _.Id

part1 sample // 8
part1 input // 2727

let calculateGamePower game = game.Red * game.Green * game.Blue

let part2 data =
    splitByLine data
    |> Seq.map parseGame
    |> Seq.sumBy calculateGamePower

part2 sample // 2286
part2 input // 56580

bench (fun () -> part2 input)
