#r "nuget: FsHttp"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
open FSharp.Text.RegexProvider

let sample =
    """
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""

let input = getInput 2023 4

type Card =
    { Id: int
      WinningNumbers: string []
      PlayedNumbers: string [] }

type ParseId = Regex< @"Card\s+(?<Id>\d+):" >
type ParseWinners = Regex< @": (?<Winners>.*) \|" >
type ParsePlayed = Regex< @"\|\s+(?<Played>.*)$" >

let parseCard line =
    let winning =
        ParseWinners()
            .TypedMatch(line)
            .Winners.Value.Split(" ", StringSplitOptions.RemoveEmptyEntries)
        |> Array.map (fun w -> w.Trim())

    let played =
        ParsePlayed()
            .TypedMatch(line)
            .Played.Value.Split(" ", StringSplitOptions.RemoveEmptyEntries)
        |> Array.map (fun w -> w.Trim())

    { Id = int (ParseId().TypedMatch(line).Id.Value)
      WinningNumbers = winning
      PlayedNumbers = played }

parseCard "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

let countWinners card =
    let played = card.PlayedNumbers |> Set.ofArray
    let winners = card.WinningNumbers |> Set.ofArray

    Set.intersect played winners |> Set.count

let part1 data =
    splitByLine data
    |> Array.map parseCard
    |> Array.sumBy (fun card ->
        match (countWinners card) with
        | 0 -> 0
        | n -> (1 <<< (n - 1)))

part1 sample // 13
part1 input // 21959

let part2 data =
    let allCards = splitByLine data |> Array.map parseCard

    let winningCounts =
        allCards
        |> Array.map (fun card -> card.Id, countWinners card)
        |> dict

    let rec countCards toProcess count =
        match toProcess with
        | nextId :: tail ->
            let winCount = winningCounts[nextId]

            let newCards =
                [ nextId + 1 .. (nextId + winCount) ]
                |> Seq.toList

            countCards (tail @ newCards) (count + 1)
        | [] -> count

    let initProcess =
        allCards
        |> Array.map (fun card -> card.Id)
        |> Array.toList

    countCards initProcess 0

part2 sample // 30?
part2 input // ?
