# Actix Web Examples

collection of examples from official documentation, my experimentation, and other sources

## How to run

### without docker

1. (optional) install diesel ``cargo install diesel-cli``
2. install postgresql
3. add your database url to env
`DATABASE_URL`
or
create .env file for example :
`DATABASE_URL=postgres://user:pass@localhost/articles`
4. run

``` bash
RUST_BACKTRACE=1 RUST_LOG=actix_web=info cargo run
```

### Docker

edit your docker configuration accordingly, then

``` bash
makefile watch
```

## migration

``` bash
diesel migration run
```

``` bash
diesel migration redo
```

## Test

run cargo test

``` bash
cargo test
```
