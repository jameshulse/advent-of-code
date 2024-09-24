module Advent

open System
open System.IO
open System.Diagnostics
open FsHttp
open FsHttp.Response

(* Helper methods *)
let getInput year day =
    let session = File.ReadAllText ".session"
    let url = $"https://adventofcode.com/%d{year}/day/%d{day}/input"

    http {
        GET url
        header "Cookie" ($"session=%s{session}")
    }
    |> Request.send
    |> toText

let split (on: string) (str: string) =
    str.Split(
        on,
        StringSplitOptions.RemoveEmptyEntries
        ||| StringSplitOptions.TrimEntries
    )

let splitByLine (data: string) = split "\n" data

let splitByEmptyLines (data: string) = split "\n\n" data

let splitSpaces (data: string) = split " " data

let bench func =
    let sw = Stopwatch.StartNew()
    let result = func ()
    sw.Stop()
    (result, sw.Elapsed)

(* Extension methods *)
module Array =
    (* Like mapi, it passes the index to each iteration being collected *)
    let collecti collector arr =
        arr
        |> Array.mapi (fun i next -> (i, next))
        |> Array.collect (fun (i, next) -> collector i next)

module Seq =
    (* Like mapi, it passes the index to each iteration being collected *)
    let collecti collector arr =
        arr
        |> Seq.mapi (fun i next -> (i, next))
        |> Seq.collect (fun (i, next) -> collector i next)

    let cycle items =
        seq {
            while true do
                yield! items
        }

module Math =
    // See: https://jeremybytes.blogspot.com/2016/07/getting-prime-factors-in-f-with-good.html
    let primeFactors n =
        let rec getFactor num proposed acc =
            if proposed = num then
                proposed :: acc
            elif num % proposed = LanguagePrimitives.GenericZero then
                getFactor (num / proposed) proposed (proposed :: acc)
            else
                getFactor num (proposed + 1UL) acc

        getFactor n 2UL [] |> List.toArray

    (* Find the lowest common multiple of a set of numbers *)
    let lcm nums =
        nums
        |> Seq.collect primeFactors
        |> Seq.distinct
        |> Seq.fold (*) LanguagePrimitives.GenericOne
