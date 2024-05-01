# dioxus-rust-workspace

Personal workspace for learning Dioxus & Rust.

## Links

- [Dioxus GitHub](https://github.com/DioxusLabs/dioxus)
- [Dioxus Website](https://dioxuslabs.com/)
- [Rust](https://www.rust-lang.org/)

## Installing Tools

- Install [Dioxus CLI](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli)

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=setup-install-tools promptEnv=true terminalRows=10 }
# install tool dependencies

set -e

cargo install dioxus-cli
cargo install cargo-make
cargo install wasm-pack

printf "\n"
set -x

dx --version
cargo make --version
wasm-pack --version

set +x
```
