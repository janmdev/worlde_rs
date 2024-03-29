﻿namespace worlde_rs_api

open System

module Models =
    type ValidateResponse(letter, state) =
        member this.Letter = letter
        member this.State = state
    
    type ResetRequest = 
     {
        src: Guid
        dst: Guid
     }

    type ValidateRequest = 
     {
        guid: Guid
        word: String
     }

     
