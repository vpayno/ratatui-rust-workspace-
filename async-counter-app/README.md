# [async-counter-app](https://ratatui.rs/tutorials/async-counter-app/)

Simple counter app.

## Commands

Use [bacon](https://github.com/Canop/bacon) to watch the project.

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=async-counter-app-watch promptEnv=true terminalRows=20 }
# run bacon

set -e

stty rows "${LINES:-25}"
stty cols "${COLUMNS:-80}"

printf "PWD: %s\n" "${PWD}"
printf "\n"

if [[ ! -f bacon.toml ]]; then
    bacon --init
    printf "\n"
fi

printf "Please select a bacon subcommand:\n"
declare subcommand
subcommand="$(gum choose --limit=1 --selected="none" none check-all clippy test | sed -r -e 's/^none$//g')"
printf "\n"

# don't quote subcommand
bacon ${subcommand}
```

Add project dependencies.

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=async-counter-app-add-deps promptEnv=true terminalRows=20 }
# install/update debian and cargo deps

set -e

stty rows "${LINES:-25}"
stty cols "${COLUMNS:-80}"

printf "PWD: %s\n" "${PWD}"
printf "\n"

: sudo nala install -y --no-autoremove xxx

printf "\n"

cargo add ratatui
cargo add crossterm --features=event-stream
cargo add color-eyre
cargo add tokio --features=full
cargo add tokio-util
cargo add futures
```

Run the project.

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=async-counter-app-run promptEnv=true terminalRows=20 }
# run the project

set -e

stty rows "${LINES:-25}"
stty cols "${COLUMNS:-80}"

printf "PWD: %s\n" "${PWD}"
printf "\n"

reset
cargo run
```

