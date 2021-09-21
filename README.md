# wasi-witx-sandbox

This is a sandbox for creating witx specs and bindings with interface-types in a devcontainer enabled repository.

## devcontainer

This project uses VS Code [devcontainers](https://code.visualstudio.com/docs/remote/containers).

All of the dependencies needed to create a Rust based witx-bindgen project are included in the devcontainer.

If this is your first time using a development container, please follow the [getting started steps](https://aka.ms/vscode-remote/containers/getting-started).

1. Clone the repository and open in VS Code
1. Press F1 and select the Remote-Containers: Open Folder in Container... command.

## WASI

[WASI](https://wasi.dev/) is a modular system interface for WebAssembly. Checkout this [blog post](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/) for an excellent overview and introduction. This system interface securely
and portability provides an interface to run WebAssembly modules [outside of the Web](https://webassembly.org/docs/non-web/).

## Interface types

A fantastic overview of the [interface types proposal](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) is this [blog post](https://hacks.mozilla.org/2019/08/webassembly-interface-types/).

## witx-bindgen

This project is a bindings generator framework for WebAssembly programs and embeddings of WebAssembly.
This works with *.witx files which describe the interface of a module, either imported or exported.

We will use witx-bindgen to compile a Rust function to WebAssembly, and generate Rust bindings
to import the WASI API that described with *.witx.

This uses the Wasmtime runtime to provide WASI functionality as a guest program.

## Walkthrough

```bash
cd component
cargo build
cd ..
cargo run
# got: 3
```
