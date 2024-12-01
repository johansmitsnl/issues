# Issues

This is just a repo for reporting issues.

This is linked for 2 issues:

* https://github.com/surrealdb/surrealkv/issues/136
* https://github.com/surrealdb/surrealdb/issues/4898

## How to reproduce

Start a server like: `$ surreal start --user root --pass root --bind 0.0.0.0:8001 surrealkv://db`

Run the script: `$ cargo run`

Observe the error:

```
called `Result::unwrap()` on an `Err` value: Api(Query("The query was not executed due to a failed transaction. There was a problem with a datastore transaction: Transaction read conflict"))
```