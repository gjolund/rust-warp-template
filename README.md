OpenFaaS Rust Warp Server template
=============================================

An OpenFaaS of-watchdog template written for the Rust Warp server library.

## Installation

```sh
$ faas template pull https://github.com/austinrivas/rust-warp-template
$ faas new --list
Languages available as templates:
- rust-warp
```

## Create Function

```sh
faas new <name> --lang rust-warp
```

## Testing

```sh
cargo test --manifest-path ./template/rust-warp/main/Cargo.toml
cargo test --manifest-path ./template/rust-warp/function/Cargo.toml
```

### Format

```sh
cargo fmt --manifest-path ./template/rust-warp/main/Cargo.toml
cargo fmt --manifest-path ./template/rust-warp/function/Cargo.toml
```

### Linting

```sh
cargo clippy --manifest-path ./template/rust-warp/main/Cargo.toml
cargo clippy --manifest-path ./template/rust-warp/function/Cargo.toml
```

## Usage

This template provides a thin wrapper around the [Rust Warp Server](https://github.com/seanmonstar/warp). The wrapper implementation closely mirrors the [example server](https://github.com/seanmonstar/warp#example).

### [Example Handler](https://github.com/austinrivas/rust-warp-template/blob/master/template/rust-warp/function/src/lib.rs)
### [Wrapper](https://github.com/austinrivas/rust-warp-template/blob/master/template/rust-warp/main/src/main.rs)

## Example Function

A working example of this function template can be found [here](https://github.com/austinrivas/openfaas_rust-warp_func).

## Extras

### [rust-warp-builder](https://hub.docker.com/r/austinrivas/rust-warp-builder/dockerfile)

A builder Docker image that pre-compiles the included dependencies of the `rust-warp` template.

Pre-compiling dependencies in this manner speeds up function builds by a significant amount.

### [rust-warp-runner](https://hub.docker.com/r/austinrivas/rust-warp-runner/dockerfile)

A runner Docker alpine image that is optimized to run OpenFaaS rust binaries with minimal dependencies.

### [rust-warp-okteto](https://hub.docker.com/r/austinrivas/rust-warp-okteto/dockerfile)

Based on `rust-warp-builder` this image is designed to run `rust-warp` OpenFaaS functions on the [Okteto](https://okteto.com/) remote development platform. It includes pre-compiled base dependencies and additional configuration to optimize it for okteto. For additional information on how to configure a function for okteto see [openfaas_rust-warp_func](https://github.com/austinrivas/openfaas_rust-warp_func).

### Actions

This repo includes [two github actions](https://github.com/austinrivas/rust-warp-template/blob/master/.github/workflows) to test / lint / format code in the main wrapper and function.
