#r "nuget: FsHttp"
#load "Advent.fs"

open System
open Advent
open System.Text.RegularExpressions

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
    { id: int
      red: int
      green: int
      blue: int }

// type Reveal =
//     | Red of int
//     | Green of int
//     | Blue of int

// let (|Red|Green|Blue|) ParseReveal reveal =

let parseGame line =
    let idSearch =
        Regex("Game (\\d+):", RegexOptions.Compiled)
            .Match(line)

    let id = int idSearch.Groups[1].Value

    let rounds = line.Substring(line.IndexOf(": ") + 2).Split("; ")

    let mutable maxRed = 0
    let mutable maxGreen = 0
    let mutable maxBlue = 0

    for round in rounds do
        for reveal in round.Split(", ") do
            let ballSearch =
                Regex(@"(\d+) (\w+)", RegexOptions.Compiled)
                    .Match(reveal)

            let count = int ballSearch.Groups[1].Value
            let colour = ballSearch.Groups[2].Value

            match colour with
            | "red" when count > maxRed -> maxRed <- count
            | "green" when count > maxGreen -> maxGreen <- count
            | "blue" when count > maxBlue -> maxBlue <- count
            | _ -> ignore ()

    { id = id
      red = maxRed
      green = maxGreen
      blue = maxBlue }

let part1 data =
    let maxRed = 12
    let maxGreen = 13
    let maxBlue = 14

    splitByLine data
    |> Seq.map parseGame
    |> Seq.filter (fun (game) ->
        game.red <= maxRed
        && game.green <= maxGreen
        && game.blue <= maxBlue)
    |> Seq.map (fun (game) -> game.id)
    |> Seq.sum

part1 sample // 8?
part1 input // 2727

let part2 data =
    splitByLine data
    |> Seq.map parseGame
    |> Seq.map (fun (game) -> game.red * game.green * game.blue)
    |> Seq.sum

part2 sample // 2286
part2 input // ?
