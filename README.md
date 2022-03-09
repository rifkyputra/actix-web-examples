# Actix Web Examples

collection of examples from official documentation and other sources

## How to run

1. (optional) install diesel ``cargo install diesel-cli``
2. install postgresql
3. add  your database url to env
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
