namespace worlde_rs_api.Controllers

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
    member this.Get(guid: Guid) =
        let rng = System.Random()
        match sessionDict.ContainsKey(guid) with
            | false -> sessionDict.Add(guid,validWords[rng.Next(validWords.Length)])
            | true -> ()
        sessionDict[guid]
        
    [<HttpPost("")>]
    member this.Post([<FromBody>] request: ValidateRequest) = 
        let toValidate = sessionDict[request.guid]
        let mutable responseList = new List<ValidateResponse>()
        for i in 0..4 do
            if request.word[i] = toValidate[i] 
            then 
                responseList.Add(ValidateResponse(request.word[i],2))
            else if request.word.Any(fun p -> p = toValidate[i])
            then responseList.Add(ValidateResponse(request.word[i],1))
            else responseList.Add(ValidateResponse(request.word[i],0))
        responseList

