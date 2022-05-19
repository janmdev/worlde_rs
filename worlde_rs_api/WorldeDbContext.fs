namespace worlde_rs_api

open Microsoft.EntityFrameworkCore
open System.Linq
open worlde_rs_api.DbModels

module WorldeDbContext =

    type WorldeDbContext(options: DbContextOptions<WorldeDbContext>) =
        inherit DbContext(options)

        [<DefaultValue>]
        val mutable Words : DbSet<Word>
        member public this._Words    with    get() = this.Words
                                     and     set value = this.Words <- value

        //returns if the Item exists 
        member this.WordExist (id:int) = this.Words.Any(fun x -> x.Id = id)

        //Returns the Item with the given id
        member this.GetWord (id:int) = this.Words.Find(id)

        let Initialize (context : WorldeDbContext) =
            //context.Database.EnsureDeleted() |> ignore //Deletes the database
            context.Database.EnsureCreated() |> ignore //check if the database is created, if not then creates it
            //default Items for testing
            let words : Word[] = 
                [|
                    { Id = 1; Value = "THESE" }
                    { Id = 2; Value = "THERE"  }
                    { Id = 3; Value = "OTHER" }
                |]

            if not(context.Words.Any()) then
                    context.Words.AddRange(words) |> ignore
                    context.SaveChanges() |> ignore  