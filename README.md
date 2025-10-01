# Puck Sigil

A simple git tagger: read package file from provided configuration file and
collect version from it.

## Contribute

You can build and test binary locally by run:

```shell
cargo run
cargo build
```

or

```shell
cargo test
```

## Install

```shell
cargo install puck-sigil
```

and after you can run:

```shell
psigil
```

## Debug

```shell
RUST_LOG=debug cargo run
```

or

```shell
RUST_LOG=debug psigil
```

## Configuration

Please see `config.json`.

You can provide path to `config.json` with command line arg `--config` or
with env value `PSIGIL_CONFIG`.
