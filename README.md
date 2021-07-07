# Anicca

Get the difference between two OpenAPI descriptions. **Still in heavy development, use at your own risk.**

## Usage

```shell
$ cargo run --bin cli diff fixtures/pet-store.json fixtures/pet-store-changed.json --format json
```

## Documentation

[Docs.rs](https://docs.rs/anicca)

## Limitations

  - Currently expects a fully dereferenced OpenAPI document. $ref support will come.
  - The diff is a work in progress, there are currently many unhandled properties and objects, but I'm quickly working through them.
