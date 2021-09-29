# wasi-data-rust

This project implements portions of the [wasi-data](https://github.com/singlestore-labs/wasi-data) specification in rust. It contains the following crates:

* [wasi-interface-gen](./crates/wasi-interface-gen): convenience macro for compiling rust functions and types to wasm modules using the canonical abi for wasi interface types.
* [example-wasm](./crates/example-wasm): example crate using `wasi-interface-gen` to compile a wasm module with interface types.
* [example-wasmtime-host](./crates/example-wasmtime-host): example of using wasmtime to load and run the `example-wasm` crate.

## devcontainer

This project uses VS Code [devcontainers](https://code.visualstudio.com/docs/remote/containers).

If this is your first time using a development container, please follow the [getting started steps](https://aka.ms/vscode-remote/containers/getting-started).

1. Clone the repository and open in VS Code
2. Press F1 and select the Remote-Containers: Open Folder in Container... command.

## WASI

[WASI](https://wasi.dev/) is a modular system interface for WebAssembly. Checkout this [blog post](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/) for an excellent overview and introduction. This system interface securely
and portability provides an interface to run WebAssembly modules [outside of the Web](https://webassembly.org/docs/non-web/).

## Interface types

A fantastic overview of the [interface types proposal](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) is this [blog post](https://hacks.mozilla.org/2019/08/webassembly-interface-types/).

## Walkthrough

```bash
cargo run
got: [SimpleValue { i: 20 }]
got: [SplitOutput { c: "hello" }, SplitOutput { c: "how" }, SplitOutput { c: "are" }, SplitOutput { c: "you" }]
got: [UserResult { id: 2, username: "lucy", email: "lucy@singlestore.com", phone: "555-123-4567" }, UserResult { id: 4, username: "bob", email: "bob@gmail.com", phone: "555-123-4567" }]
```
