namespace worlde_rs_api.Controllers

open System
open System.Collections.Generic
open System.Linq
open System.Threading.Tasks
open Microsoft.AspNetCore.Mvc
open Microsoft.Extensions.Logging
open worlde_rs_api
open worlde_rs_api.WorldeDbContext
open Models

[<ApiController>]
[<Route("[controller]")>]
type WordsController () =
    inherit ControllerBase()

    //new(context: WorldeDbContext) as this =
    //    WordsController () then
    //    this._Context <- context

    //[<DefaultValue>]
    //val mutable _Context : WorldeDbContext

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

    [<NonAction>]
    member this.getRandomWord() =
        let rng = System.Random()
        validWords[rng.Next(validWords.Length)];
        //this._Context.Words.Find(rng.Next(this._Context.Words.Count())).Value

    [<NonAction>]
    member this.checkWord(word: string) =
        validWords.Contains(word);
        //this._Context.Words.Any( fun s -> s.Value = word)

    [<HttpPut("sessions/{guid}")>]
    member this.Put(guid: Guid) =
        match sessionDict.ContainsKey(guid) with
            | false -> sessionDict.Add(guid,this.getRandomWord())
            | true -> ()

    [<HttpDelete("sessions/{guid}")>]
    member this.Delete(guid: Guid) =
        sessionDict.Remove(guid)
        
    [<HttpPost("")>]
    member this.Post([<FromBody>] request: ValidateRequest) = 
        if this.checkWord(request.word)
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
                this.Ok(responseList) :> IActionResult
            else
                this.NotFound("Session not foud") :> IActionResult
        else
           this.BadRequest("Word not in dictionary") :> IActionResult

    [<HttpPost("sessions/reset")>]
    member this.Reset([<FromBody>] request: ResetRequest) =
        let rng = System.Random()
        if sessionDict.ContainsKey(request.src)
        then
            sessionDict.Remove(request.src);
            sessionDict.Add(request.dst,this.getRandomWord());
            this.Ok() :> IActionResult
        else
            sessionDict.Add(request.dst,this.getRandomWord());
            this.Ok() :> IActionResult

