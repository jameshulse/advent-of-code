#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
open FSharp.Text.RegexProvider
open FSharpPlus

let sample =
    """
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"""

let input = getInput 2023 7

type Hand =
    { Cards: char []
      HandRank: int
      Bid: int }

let parseLine (line: string) =
    let parts = line.Split(" ")
    let cards = parts[0] |> String.toArray

    let groupings =
        cards
        |> Array.groupBy (fun card -> card)
        |> Seq.map (fun (card, group) -> group.Length)
        |> Seq.sortDescending
        |> Seq.toList

    let score =
        match groupings with
        | 5 :: _ -> 7
        | 4 :: _ -> 6
        | 3 :: 2 :: _ -> 5
        | 3 :: _ -> 4
        | 2 :: 2 :: _ -> 3
        | 2 :: _ -> 2
        | _ -> 1

    { Cards = cards
      HandRank = score
      Bid = int parts[1] }

parseLine "32T3K 765"
parseLine "23332 10"

let cardScore c =
    Array.IndexOf("23456789TJQKA" |> String.toArray, c)

let compareHands (a: Hand) (b: Hand) =
    if a.HandRank > b.HandRank then
        1
    elif a.HandRank = b.HandRank then
        let rec byCard index =
            if index = 5 then
                0
            elif a.Cards[index] = b.Cards[index] then
                byCard (index + 1)
            else
                (cardScore a.Cards[index])
                - (cardScore b.Cards[index])

        byCard 0
    else
        -1

compareHands (parseLine "KK677 28") (parseLine "KTJJT 220")
compareHands (parseLine "KK677 28") (parseLine "QQQJA 483")

let part1 data =
    splitByLine data
    |> Array.map parseLine
    |> Array.sortWith (fun a b -> compareHands a b)
    |> Array.mapi (fun i hand -> hand.Bid * (i + 1))
    |> Array.sum

part1 sample // 6440
part1 input // 255048101

let part2 data = ()

part2 sample
part2 input
