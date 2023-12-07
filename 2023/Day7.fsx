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

type JokerRule =
    | NoJoker
    | Joker

type Hand =
    { Cards: char list
      HandRank: int
      Bid: int }

let getHandRankNoJokers (cards: char list) =
    let groupings =
        cards
        |> List.groupBy (fun card -> card)
        |> List.map (fun (_, group) -> group.Length)
        |> List.sortDescending

    match groupings with
    | 5 :: _ -> 7
    | 4 :: _ -> 6
    | 3 :: 2 :: _ -> 5
    | 3 :: _ -> 4
    | 2 :: 2 :: _ -> 3
    | 2 :: _ -> 2
    | _ -> 1

let getHandRankWithJokers (cards: char list) =
    let groupings =
        cards
        |> List.groupBy (fun card -> card)
        |> List.filter (fun (card, _) -> card <> 'J')
        |> List.map (fun (_, group) -> group.Length)
        |> List.sortDescending

    let jokers =
        cards
        |> List.filter (fun c -> c = 'J')
        |> List.length

    printfn $"{jokers}"

    if jokers > 0 then
        match groupings with
        // Five of a kind
        | 5 :: _ -> 7
        | _ when jokers = 5 -> 7
        // Jokers making up to 5 or 4 of a kind
        | n :: _ when n + jokers = 5 -> 7
        | n :: _ when n + jokers = 4 -> 6
        // Full house
        | 2 :: 2 :: _ -> 5
        // 3 of a kind
        | n :: _ when n + jokers = 3 -> 4
        // Two pair
        | 2 :: 1 :: _ -> 3
        // Fall back to one pair
        | _ -> 2
    else
        getHandRankNoJokers cards

// Five of a kind -> 7
// Four of a kind -> 6
// Full house -> 5
// Three of a kind -> 4
// Two pair -> 3
// Pair -> 2

getHandRankWithJokers ("553J5" |> String.toList) // 6
getHandRankWithJokers ("JK949" |> String.toList) // 4
getHandRankWithJokers ("JJJJJ" |> String.toList) // 7
getHandRankWithJokers ("J8888" |> String.toList) // 7
getHandRankWithJokers ("QJQJJ" |> String.toList) // 7
getHandRankWithJokers ("J3A33" |> String.toList) // 6
getHandRankWithJokers ("JT4T4" |> String.toList) // 5

let parseLine (line: string) jokerRule =
    let parts = line.Split(" ")
    let cards = parts[0] |> String.toList

    { Cards = cards
      HandRank =
        match jokerRule with
        | NoJoker -> getHandRankNoJokers cards
        | Joker -> getHandRankWithJokers cards
      Bid = int parts[1] }

parseLine "32T3K 765" Joker // One pair (2)
parseLine "KK677 10" Joker // Two pair (3)
parseLine "T55J5 10" Joker // Two pair (3)

let cardScore c jokerRule =
    let cardOrder =
        match jokerRule with
        | Joker -> "J23456789TQKA"
        | NoJoker -> "23456789TJQKA"
        |> String.toArray

    Array.IndexOf(cardOrder, c)

let compareHands (a: Hand) (b: Hand) jokerRule =
    if a.HandRank > b.HandRank then
        1
    elif a.HandRank = b.HandRank then
        let rec byCard index =
            if index = 5 then
                0
            elif a.Cards[index] = b.Cards[index] then
                byCard (index + 1)
            else
                (cardScore a.Cards[index] jokerRule)
                - (cardScore b.Cards[index] jokerRule)

        byCard 0
    else
        -1

let part1 data =
    splitByLine data
    |> Array.map (fun line -> parseLine line NoJoker)
    |> Array.sortWith (fun a b -> compareHands a b NoJoker)
    |> Array.mapi (fun i hand -> hand.Bid * (i + 1))
    |> Array.sum

part1 sample // 6440
part1 input // 255048101

let part2 data =
    splitByLine data
    |> Array.map (fun line -> parseLine line Joker)
    |> Array.sortWith (fun a b -> compareHands a b Joker)
    |> Array.mapi (fun i hand -> hand.Bid * (i + 1))
    |> Array.sum

part2 sample // 5905 (correct on sample)
part2 input // 253995462 (incorrect on input data!)
