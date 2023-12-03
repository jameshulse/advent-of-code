#r "nuget: FsHttp"
#load "Advent.fs"

open Advent
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

type PartNumber =
    { Value: int
      Y: int
      XStart: int
      XEnd: int }

let matchNumbers = Regex(@"\d{1,3}", RegexOptions.Compiled)

let findPartNumbers lines =
    lines
    |> Array.collecti (fun y line ->
        matchNumbers.Matches(line)
        |> Seq.toArray
        |> Array.map (fun number ->
            { Value = int number.Value
              Y = y
              XStart = number.Index
              XEnd = number.Index + number.Length - 1 }))

type Symbol = { Glyph: char; X: int; Y: int }

let matchSymbols = Regex(@"[^\d\.]{1}", RegexOptions.Compiled)

let findSymbols lines =
    lines
    |> Seq.collecti (fun y line ->
        matchSymbols.Matches(line)
        |> Seq.map (fun symbol ->
            { Glyph = char symbol.Value
              X = symbol.Index
              Y = y }))
    |> Seq.toArray

let findAdjacentPartNumbers allPartNumbers symbol =
    allPartNumbers
    |> Array.filter (fun (part: PartNumber) ->
        symbol.Y >= part.Y - 1
        && symbol.Y <= part.Y + 1
        && symbol.X >= part.XStart - 1
        && symbol.X <= part.XEnd + 1)

let part1 data =
    let lines = splitByLine data
    let symbols = findSymbols lines
    let adjacentPartFinder = findPartNumbers lines |> findAdjacentPartNumbers

    symbols
    |> Seq.collect (fun symbol -> adjacentPartFinder symbol)
    |> Seq.sumBy (fun part -> part.Value)

part1 sample // 4361
part1 input // 554003

let part2 data =
    let lines = splitByLine data
    let parts = findPartNumbers lines

    findSymbols lines
    |> Seq.filter (fun symbol -> symbol.Glyph = '*')
    |> Seq.map (findAdjacentPartNumbers parts)
    |> Seq.filter (fun adjacent -> adjacent.Length = 2)
    |> Seq.sumBy (fun adjacent -> adjacent[0].Value * adjacent[1].Value)

part2 sample // 467835
part2 input // 87263515
