module Helpers

open Microsoft.FSharp.Collections

exception ParsingError of string

let readLines (filePath:string):seq<string> =  System.IO.File.ReadLines(filePath)
