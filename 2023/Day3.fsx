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

type Part =
    { Value: int
      Y: int
      XStart: int
      XEnd: int }

let matchNumbers = Regex(@"\d{1,3}", RegexOptions.Compiled)

let findNumbersOnLine line =
    matchNumbers.Matches(line)
    |> Seq.map (fun number -> (int number.Value, number.Index, number.Length))
    |> Seq.toArray

let findParts lines =
    lines
    |> Array.mapi (fun y line -> (y, findNumbersOnLine line))
    |> Array.collect (fun (y, numbers) ->
        numbers
        |> Array.map (fun (value, ind, len) ->
            { Value = value
              Y = y
              XStart = ind
              XEnd = ind + len - 1 }))

type Symbol = { Value: string; X: int; Y: int }

let matchSymbols = Regex(@"[^\d\.]{1}", RegexOptions.Compiled)

let findSymbolsOnLine line =
    matchSymbols.Matches(line)
    |> Seq.map (fun symbol -> (symbol.Value, symbol.Index))
    |> Seq.toArray

let findSymbols lines =
    lines
    |> Array.mapi (fun y line -> (y, findSymbolsOnLine line))
    |> Array.collect (fun (y, symbols) ->
        symbols
        |> Array.map (fun (symbol, x) -> { Value = symbol; X = x; Y = y }))

let findAdjacentParts allParts symbol =
    allParts
    |> Array.filter (fun (part: Part) ->
        symbol.Y >= part.Y - 1
        && symbol.Y <= part.Y + 1
        && symbol.X >= part.XStart - 1
        && symbol.X <= part.XEnd + 1)

let part1 data =
    let lines = splitByLine data
    let symbols = findSymbols lines
    let parts = findParts lines

    symbols
    |> Array.collect (fun symbol -> findAdjacentParts parts symbol)
    |> Array.sumBy (fun part -> part.Value)

part1 sample // 4361
part1 input // 554003

let part2 data =
    let lines = splitByLine data

    let gears =
        findSymbols lines
        |> Array.filter (fun s -> s.Value = "*")

    let parts = findParts lines

    gears
    |> Array.map (findAdjacentParts parts)
    |> Array.filter (fun adjacent -> adjacent.Length = 2)
    |> Array.sumBy (fun adjacent -> adjacent[0].Value * adjacent[1].Value)

part2 sample // 467835
part2 input // 87263515
