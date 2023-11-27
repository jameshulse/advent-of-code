module Advent

open FsHttp
open FsHttp.Response
open System.IO

let getInput year day =
    let session = File.ReadAllText ".session"
    let url = $"https://adventofcode.com/%d{year}/day/%d{day}/input"

    http {
        GET url
        header "Cookie" ($"session=%s{session}")
    }
    |> Request.send
    |> toText
