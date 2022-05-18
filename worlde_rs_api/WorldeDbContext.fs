namespace worlde_rs_api

open Microsoft.EntityFrameworkCore
open System.Linq
open worlde_rs_api.DbModels

module WorldeDbContext =

    type WorldeDbContext(options: DbContextOptions<DbContext>) =
        inherit DbContext(options)

        [<DefaultValue>]
        val mutable Words : DbSet<Word>
        member public this._Words    with    get() = this.Words
                                     and     set value = this.Words <- value