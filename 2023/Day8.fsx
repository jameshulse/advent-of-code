﻿#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open Advent
open FSharp.Text.RegexProvider
open FSharpPlus
open System.Collections.Generic

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

let countStepsToFinish endNode (allNodes: IDictionary<string, string * string>) instructions node =
    let mutable currentNode = node

    let count =
        instructions
        |> Seq.takeWhile (fun nextInstruction ->
            let nextNode =
                match nextInstruction with
                | 'L' -> fst allNodes[currentNode]
                | 'R' -> snd allNodes[currentNode]
                | _ -> failwith "Invalid instruction"

            currentNode <- nextNode

            not (String.endsWith endNode currentNode))
        |> Seq.length

    count + 1

let part1 data =
    let parts = data |> splitByEmptyLines
    let instructions = parts[0] |> Seq.cycle
    let allNodes = parseNodes parts[1]

    countStepsToFinish "ZZZ" allNodes instructions "AAA"

part1 sample1 // 2
part1 sample2 // 6
part1 input // 1787

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
    let instructions = parts[0] |> Seq.cycle
    let allNodes = parseNodes parts[1]

    let startingNodes =
        allNodes.Keys
        |> Seq.filter (fun n -> (String.endsWith "A" n))
        |> Seq.toArray

    // Find "lowest common multiple" for the step count per starting point
    // See: https://www.wolframalpha.com/input?i=lcm%2817873%2C+19631%2C+17287%2C+12599%2C+21389%2C+20803%29
    startingNodes
    |> Array.map (fun n -> countStepsToFinish "Z" allNodes instructions n)
    |> Array.map uint64
    |> Math.lcm

part2 sample3 // 6
bench (fun () -> part2 input) // 15746133679061
