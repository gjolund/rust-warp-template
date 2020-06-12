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

### Linting

## Usage

This template provides a thin wrapper around the [Rust Warp Server](https://github.com/seanmonstar/warp). The wrapper implementation closely mirrors the [example server](https://github.com/seanmonstar/warp#example).

### [Example Handler]()
### [Wrapper]()

## Example Function

A working example of this function template can be found [here](https://github.com/austinrivas/openfaas_rust-warp_func).

## Extras