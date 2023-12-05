#r "nuget: FsHttp"
#r "nuget: FSharpPlus"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
open FSharpPlus
open FSharp.Text.RegexProvider

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

// type Number =
//     | Seed of uint
//     | Soil of uint
//     | Fertilizer of uint
//     | Water of uint
//     | Light of uint
//     | Temperature of uint
//     | Humidity of uint
//     | Location of uint

// type Number = Value of uint<'a>

// type Seed = Seed of uint
// type Soil = Soil of uint
// type Fertilizer = Fertilizer of uint
// type Water = Water of uint
// type Light = Light of uint
// type Temperature = Temperature of uint
// type Humidity = Humidity of uint
// type Location = Location of uint

// [<Measure>]
// type seed

// [<Measure>]
// type soil

// [<Measure>]
// type fertilizer

// [<Measure>]
// type water

// [<Measure>]
// type light

// [<Measure>]
// type temperature

// [<Measure>]
// type humidity

// [<Measure>]
// type location

// type MappingType =
//     | SeedToSoil
//     | SoilToFertilizer
//     | FertilizerToWater
//     | WaterToLight
//     | LightToTemperature
//     | TempoeratureToHumidity
//     | HumidityToLocation

type Range =
    { Start: uint
      Size: uint }
    member this.Contains n =
        this.Start <= n && n <= this.Start + this.Size

({ Start = 98u; Size = 2u }).Contains(99u)
({ Start = 98u; Size = 2u }).Contains(25u)

module Mapping =
    type T =
        { From: Range
          To: Range }
        member this.Convert from =
            match from with
            | n when this.From.Contains(n) -> n - this.From.Start + this.To.Start
            | n -> n

    let create fromStart toStart size =
        { From = { Start = fromStart; Size = size }
          To = { Start = toStart; Size = size } }

(Mapping.create 98u 50u 2u).Convert(99u)
(Mapping.create 0u 39u 15u).Convert(14u)

let convertThroughSection mappings value =
    let validMapping =
        mappings
        |> Array.filter (fun (m: Mapping.T) -> m.From.Contains(value))
        |> Array.tryHead

    match validMapping with
    | Some (m) -> m.Convert(value)
    | None -> value

let parseSections rawSections =
    let parseSection section =
        splitByLine section
        |> Array.skip (1) (* Skip the name *)
        |> Array.map (fun line -> line.Split(" ") |> Array.map uint)
        |> Array.map (fun parts -> Mapping.create parts[1] parts[0] parts[2])

    rawSections |> Array.map parseSection


let part1 data =
    let rawSections = splitByEmptyLines data

    let seeds =
        rawSections[ 0 ].Replace("seeds: ", "").Split(" ")
        |> Array.map uint

    let mappingsBySection = parseSections rawSections[1..]

    seeds
    |> Array.map (fun seed ->
        mappingsBySection
        |> Array.fold (fun v section -> convertThroughSection section v) seed)
    |> Array.min

part1 sample // 35
part1 input // 825516882

let part2 data = ()

part2 sample
part2 input
