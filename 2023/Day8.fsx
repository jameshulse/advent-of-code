#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
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

type NodeRegex = Regex< @"^(?<Name>.{3}) = \((?<Left>.{3}), (?<Right>.{3})\)$" >

let parseNodes nodeList =
    nodeList
    |> splitByLine
    |> Array.map (fun line ->
        let node = NodeRegex().TypedMatch(line)
        let name = node.Name.Value

        name, (node.Left.Value, node.Right.Value))
    |> dict

let part1 data =
    let parts = data |> splitByEmptyLines
    let instructions = parts[0] |> Seq.repeat
    let allNodes = parseNodes parts[1]

    let mutable steps = 0
    let mutable currentNode = "AAA"

    while currentNode <> "ZZZ" do
        let nextInstruction = instructions |> Seq.skip steps |> Seq.head

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

let sample3 =
    """
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"""

let part2 data =
    let parts = data |> splitByEmptyLines
    let instructions = parts[0] |> Seq.repeat
    let allNodes = parseNodes parts[1]

    let startNodes =
        allNodes.Keys
        |> Seq.filter (fun n -> (String.endsWith "A" n))
        |> Seq.toArray

    let countStepsToFinish node =
        let mutable steps = 0
        let mutable currentNode = node

        while not (String.endsWith "Z" currentNode) do
            let nextInstruction = instructions |> Seq.skip steps |> Seq.head

            let nextNode =
                match nextInstruction with
                | 'L' -> fst allNodes[currentNode]
                | 'R' -> snd allNodes[currentNode]
                | _ -> failwith "Invalid instruction"

            currentNode <- nextNode
            steps <- steps + 1

        steps

    startNodes |> Array.map countStepsToFinish

// From here find the "least common multiple" of each path...

part2 sample3 // 2
part2 input // 15746133679061
