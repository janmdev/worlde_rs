namespace worlde_rs_api

open System.ComponentModel.DataAnnotations

module DbModels =

    [<CLIMutable>]
    type Word = 
        {
            Id: int
            [<Required>]
            Value: string
        }