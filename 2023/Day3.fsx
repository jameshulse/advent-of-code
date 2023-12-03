#r "nuget: FsHttp"
#r "nuget: FSharp.Text.RegexProvider"
#load "Advent.fs"

open System
open Advent
// open FSharp.Text.RegexProvider
open System.Text.RegularExpressions

let sample =
    """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

let input = getInput 2023 3

let matchSymbols = Regex(@"[^\d\.]", RegexOptions.Compiled)

let findSymbols line =
    matchSymbols.Matches(line)
    |> Seq.map (fun symbol -> symbol.Index)
    |> Seq.toArray


let matchNumbers = Regex(@"\d+", RegexOptions.Compiled)

type Part =
    { Value: int
      Y: int
      XStart: int
      XEnd: int }

let findNumbers line =
    matchNumbers.Matches(line)
    |> Seq.map (fun number -> (int number.Value, number.Index, number.Length))
    |> Seq.toArray

findNumbers "467..114.."
findNumbers "...*......"
findNumbers ".....+.58."

let partTouchesSymbol symbols part =
    let mutable touches = false

    for (symX, symY) in symbols do
        if symY >= part.Y - 1
           && symY <= part.Y + 1
           && symX >= part.XStart - 1
           && symX <= part.XEnd + 1 then
            touches <- true

    touches

let part1 data =
    let lines = splitByLine data

    let symbols =
        lines
        |> Array.mapi (fun y line -> (y, findSymbols line))
        |> Array.collect (fun (y, symbols) -> symbols |> Array.map (fun x -> (x, y)))

    let parts =
        lines
        |> Array.mapi (fun y line -> (y, findNumbers line))
        |> Array.collect (fun (y, numbers) ->
            numbers
            |> Array.map (fun (value, ind, len) ->
                { Value = value
                  Y = y
                  XStart = ind
                  XEnd = ind + len - 1 }))

    parts
    |> Array.filter (partTouchesSymbol symbols)
    |> Array.sumBy (fun part -> part.Value)

part1 sample
part1 input

let part2 data = ()

part2 sample
part2 input
