#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

// open System
open Advent
open FSharp.Text.RegexProvider
open FSharpPlus

let sample1 =
    """
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"""

let sample2 =
    """
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"""

let input = getInput 2023 8

type NodeRegex = Regex< @"(?<Name>[A-Z]{3}) = \((?<Left>[A-Z]{3}), (?<Right>[A-Z]{3})\)" >

let (|EndNode|_|) name =
    if String.endsWith name "Z" then
        Some(EndNode)
    else
        None

let part1 data =
    let parts = data |> splitByEmptyLines
    let instructions = parts[0] |> Seq.repeat

    let allNodes =
        parts[1]
        |> splitByLine
        |> Array.map (fun line ->
            let node = NodeRegex().TypedMatch(line)
            let name = node.Name.Value

            name, (node.Left.Value, node.Right.Value))
        |> dict

    let mutable steps = 0
    let mutable currentNode = "AAA"
    let mutable nextInstruction = '_'

    while currentNode <> "ZZZ" do
        nextInstruction <- instructions |> Seq.skip steps |> Seq.head

        let nextNode =
            match nextInstruction with
            | 'L' -> fst allNodes[currentNode]
            | 'R' -> snd allNodes[currentNode]
            | _ -> failwith "Invalid instruction"

        currentNode <- nextNode

        steps <- steps + 1

    steps

part1 sample1 // 2
part1 sample2 // 6
part1 input

let part2 data = ()

part2 sample1
part2 sample2
part2 input
