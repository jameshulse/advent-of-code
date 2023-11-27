#r "nuget: FsHttp"
#load "Advent.fs"

open System
open Advent

let sample =
    """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"""

let getWeights (input: string) =
    input.Split("\n\n")
    |> Seq.map (fun s -> s.Split("\n", StringSplitOptions.RemoveEmptyEntries))
    |> Seq.map (Array.map int)
    |> Seq.map (Array.sum)

let part1 input = getWeights input |> Seq.max

let day1Input = getInput 2022 1

part1 sample
part1 day1Input

let part2 input =
    getWeights input
    |> Seq.sortDescending
    |> Seq.take 3
    |> Seq.sum
    
part2 sample
part2 day1Input
