#r "nuget: FsHttp"
#r "nuget: FSharp.Text.RegexProvider"
#r "nuget: FSharpx.Extras"
#load "Advent.fs"

open FSharpx
open System
open Advent
open FSharp.Text.RegexProvider
open System.Collections.Generic

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

type Card = { Id: int; Wins: int }

type ParseLine = Regex< @"^Card\s+(?<Id>\d+):(?<Winners>.*)\|(?<Played>.*)$" >

let parseCard line =
    let parsed = ParseLine().TypedMatch(line)
    let winning = parsed.Winners.Value |> split " " |> set
    let played = parsed.Played.Value |> split " " |> set

    let wins = Set.intersect played winning |> Set.count

    { Id = int (parsed.Id.Value)
      Wins = wins }

parseCard "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

let part1 data =
    splitByLine data
    |> Array.map parseCard
    |> Array.sumBy (fun card ->
        match (card.Wins) with
        | 0 -> 0
        | n -> (1 <<< (n - 1)))

part1 sample // 13
part1 input // 21959

let part2 data =
    let allCards =
        splitByLine data
        |> Array.map parseCard
        |> Array.map (fun card -> card.Id, card)
        |> dict

    let countCache = new Dictionary<int, int>()

    let rec countCards toProcess count =
        match toProcess with
        | nextId :: tail ->
            let subCount =
                match countCache.TryGetValue(nextId) with
                | true, count -> count
                | _ ->
                    let winCount = allCards[nextId].Wins

                    let subCount' =
                        if winCount = 0 then
                            0
                        else
                            let newCards = [ nextId + 1 .. (nextId + winCount) ]

                            countCards newCards 0

                    countCache.Add(nextId, subCount')

                    subCount'

            countCards (tail) (count + subCount + 1)
        | [] -> count

    let initProcess = allCards.Keys |> Seq.toList

    countCards initProcess 0

part2 sample // 30
part2 input // 5132675
