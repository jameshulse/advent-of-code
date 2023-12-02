module Advent

open System
open System.IO
open System.Diagnostics
open FsHttp
open FsHttp.Response

let getInput year day =
    let session = File.ReadAllText ".session"
    let url = $"https://adventofcode.com/%d{year}/day/%d{day}/input"

    http {
        GET url
        header "Cookie" ($"session=%s{session}")
    }
    |> Request.send
    |> toText

let splitByLine (data: string) =
    data.Split("\n", StringSplitOptions.RemoveEmptyEntries)

let splitByEmptyLines (data: string) =
    data.Split("\n\n", StringSplitOptions.RemoveEmptyEntries)

let bench func =
    let sw = Stopwatch.StartNew()
    let result = func ()
    sw.Stop()
    (result, sw.Elapsed)
