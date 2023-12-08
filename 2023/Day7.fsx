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

let HandSize = 5

type HandCategory =
    | FiveOfAKind = 7
    | FourOfAKind = 6
    | FullHouse = 5
    | ThreeOfAKind = 4
    | TwoPair = 3
    | Pair = 2
    | HighCard = 1

type Hand =
    { Cards: char list
      Category: HandCategory
      Bid: int }

let getHandRankNoJokers (cards: char list) =
    let groupings =
        cards
        |> List.groupBy (fun card -> card)
        |> List.map (fun (_, group) -> group.Length)
        |> List.sortDescending

    match groupings with
    | 5 :: _ -> HandCategory.FiveOfAKind
    | 4 :: _ -> HandCategory.FourOfAKind
    | 3 :: 2 :: _ -> HandCategory.FullHouse
    | 3 :: _ -> HandCategory.ThreeOfAKind
    | 2 :: 2 :: _ -> HandCategory.TwoPair
    | 2 :: _ -> HandCategory.Pair
    | _ -> HandCategory.HighCard

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

    if jokers > 0 then
        match groupings with
        // Five of a kind
        | 5 :: _ -> HandCategory.FiveOfAKind
        | _ when jokers = 5 -> HandCategory.FiveOfAKind
        // Jokers making up to 5 or 4 of a kind
        | n :: _ when n + jokers = 5 -> HandCategory.FiveOfAKind
        | n :: _ when n + jokers = 4 -> HandCategory.FourOfAKind
        // Full house
        | 2 :: 2 :: _ -> HandCategory.FullHouse
        // 3 of a kind
        | n :: _ when n + jokers = 3 -> HandCategory.ThreeOfAKind
        // Two pair
        | 2 :: 1 :: _ -> HandCategory.TwoPair
        // Fall back to one pair
        | _ -> HandCategory.Pair
    else
        getHandRankNoJokers cards

let cardScore c jokerRule =
    let cardOrder =
        match jokerRule with
        | Joker -> "J23456789TQKA"
        | NoJoker -> "23456789TJQKA"

    Array.IndexOf(cardOrder |> String.toArray, c)

let compareHands (a: Hand) (b: Hand) jokerRule =
    if a.Category > b.Category then
        1
    elif a.Category = b.Category then
        [| 0..HandSize |]
        |> Array.choose (fun i ->
            if i = HandSize then
                Some(0)
            elif a.Cards[i] = b.Cards[i] then
                None
            else
                Some(
                    (cardScore a.Cards[i] jokerRule)
                    - (cardScore b.Cards[i] jokerRule)
                ))
        |> Array.head
    else
        -1

let parseLine (line: string) jokerRule =
    let parts = line.Split(" ")
    let cards = parts[0] |> String.toList

    { Cards = cards
      Category =
        match jokerRule with
        | NoJoker -> getHandRankNoJokers cards
        | Joker -> getHandRankWithJokers cards
      Bid = int parts[1] }

let calculateTotalWinnings jokerRule data =
    splitByLine data
    |> Array.map (fun line -> parseLine line jokerRule)
    |> Array.sortWith (fun a b -> compareHands a b jokerRule)
    |> Array.mapi (fun i hand -> hand.Bid * (i + 1))
    |> Array.sum

let part1 data = calculateTotalWinnings NoJoker data

part1 sample // 6440
part1 input // 255048101

let part2 data = calculateTotalWinnings Joker data

part2 sample // 5905
part2 input // 253718286
