#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
open FSharpPlus
open FSharp.Text.RegexProvider
open System.Collections.Generic


let sample =
    """
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""

let input = getInput 2023 5

type Range =
    { Start: uint
      Size: uint }

    member this.End = this.Start + this.Size

    member this.Contains n = this.Start <= n && n <= this.End

// member this.Intersects(range: Range) =
//     match range with
//     | _ when range.Contains(this.Start) -> true
//     | _ when range.Contains(this.End) -> true
//     | _ -> false

// member this.SpliceInto(range: Range) =
//     if range.Contains(this.Start) && range.Contains(this.End) then
//         [| this |]
//     elif range.Contains(this.Start) then
//         [| { Start = this.Start
//              Size = range.End - this.Start }
//            { Start = range.End
//              Size = this.End - range.End } |]
//     elif range.Contains(this.End) then
//         [| { Start = this.Start
//              Size = this.End - range.Start }
//            { Start = range.Start
//              Size = range.End - this.End } |]
//     else [| range |]

module Mapping =
    type T =
        { From: Range
          To: Range }

        member this.ConvertValue from =
            match from with
            | n when this.From.Contains(n) -> n - this.From.Start + this.To.Start
            | n -> n

    // member this.ConvertRange(range: Range) =
    //     let mapRange (r: Range) =
    //         if this.From.Contains(r) then
    //             r.S

    //     range.SpliceInto(this.From)
    //     |> Seq.map (fun r -> this.ConvertRange r)
    //     |> Seq.toArray

    let create fromStart toStart size =
        { From = { Start = fromStart; Size = size }
          To = { Start = toStart; Size = size } }

let parseSections rawSections =
    let parseSection section =
        splitByLine section
        |> Array.skip (1) (* Skip the name line *)
        |> Array.map (fun line -> line |> splitSpaces |> Array.map uint)
        |> Array.map (fun parts -> Mapping.create parts[1] parts[0] parts[2])

    rawSections |> Array.map parseSection

let part1 data =
    let rawSections = splitByEmptyLines data

    let seeds =
        rawSections[0]
        |> replace "seeds: " ""
        |> splitSpaces
        |> Array.map uint

    let mappingsBySection = parseSections rawSections[1..]

    let convertThroughSection mappings value =
        let validMapping =
            mappings
            |> tryFind (fun (m: Mapping.T) -> m.From.Contains(value))

        match validMapping with
        | Some (m) -> m.ConvertValue(value)
        | None -> value

    seeds
    |> Array.map (fun seed ->
        mappingsBySection
        |> Array.fold (fun v section -> convertThroughSection section v) seed)
    |> Array.min

part1 sample // 35
part1 input // 825516882

let part2 data =
    let rawSections = splitByEmptyLines data

    let seedRange =
        rawSections[0]
        |> replace "seeds: " ""
        |> splitSpaces
        |> Array.chunkBySize 2
        |> Array.map (fun s -> { Start = uint s[0]; Size = uint s[1] })

    let mappingsBySection = parseSections rawSections[1..]

    let convertThroughSection mappings value =
        let validMapping =
            mappings
            |> tryFind (fun (m: Mapping.T) -> m.From.Contains(value))

        match validMapping with
        | Some (m) -> m.ConvertValue(value)
        | None -> value

    seedRange
    |> Array.collect (fun r -> [| r.Start .. r.End |])
    |> Array.Parallel.map (fun seed ->
        mappingsBySection
        |> Array.fold (fun v section -> convertThroughSection section v) seed)
    |> Array.min

part2 sample // 46
bench (fun () -> part2 input) // 136096660
