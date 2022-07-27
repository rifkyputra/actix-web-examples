# Actix Web Examples

collection of examples from official documentation and other sources

## How to run

without docker :

1. (optional) install diesel ``cargo install diesel-cli``
2. install postgresql
3. add your database url to env
`DATABASE_URL`
or
create .env file for example :
`DATABASE_URL=postgres://user:pass@localhost/articles`
4. run

``` rust
RUST_BACKTRACE=1 RUST_LOG=actix_web=info cargo run
```

## migration

``` bash
diesel migration run
```

``` bash
diesel migration redo
```
