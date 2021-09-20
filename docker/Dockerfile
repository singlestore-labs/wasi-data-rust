# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.191.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

ENV WASMTIME_VERSION=0.30.0
RUN curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz \
  | tar -xJ --wildcards --no-anchored --strip-components 1 -C /usr/bin wasmtime
