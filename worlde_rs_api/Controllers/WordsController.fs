﻿namespace worlde_rs_api.Controllers

open System
open System.Collections.Generic
open System.Linq
open System.Threading.Tasks
open Microsoft.AspNetCore.Mvc
open Microsoft.Extensions.Logging
open worlde_rs_api
open Models

[<ApiController>]
[<Route("[controller]")>]
type WordsController (logger : ILogger<WordsController>) =
    inherit ControllerBase()

    let validWords =
        [|
            "WHICH"
            "THERE"
            "THEIR"
            "ABOUT"
            "WOULD"
            "THESE"
            "OTHER"
            "WORDS"
            "COULD"
            "WRITE"
            "FIRST"
            "WATER"
            "AFTER"
            "WHERE"
        |]
    static let mutable sessionDict = new Dictionary<Guid,string>()

    [<HttpPut("sessions/{guid}")>]
    member this.Put(guid: Guid) =
        let rng = System.Random()
        match sessionDict.ContainsKey(guid) with
            | false -> sessionDict.Add(guid,validWords[rng.Next(validWords.Length)])
            | true -> ()
        //sessionDict[guid]

    [<HttpDelete("sessions/{guid}")>]
    member this.Delete(guid: Guid) =
        sessionDict.Remove(guid)
        
    [<HttpPost("")>]
    member this.Post([<FromBody>] request: ValidateRequest) = 
        if validWords.Contains(request.word)
        then
            if sessionDict.ContainsKey(request.guid) 
            then
                let toValidate = sessionDict[request.guid]
                let mutable responseList = new List<uint8>()
                for i in 0..4 do
                    if request.word[i] = toValidate[i] 
                    then 
                        responseList.Add((uint8)2)
                    else if request.word.Any(fun p -> p = toValidate[i])
                    then responseList.Add((uint8)1)
                    else responseList.Add((uint8)0)
                this.Ok(responseList)
            else
                this.Ok("Session not foud")
        else
           this.Ok("Word not in dictionary")

