# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.191.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# Install Starship https://starship.rs/ for pretty prompt
RUN curl -fsSL https://starship.rs/install.sh | bash /dev/stdin --yes \
    && echo 'eval "$(starship init zsh)"' >> /root/.zshrc

ENV WASMTIME_VERSION=0.29.0
RUN curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz \
  | tar -xJ --wildcards --no-anchored --strip-components 1 -C /usr/bin wasmtime

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]