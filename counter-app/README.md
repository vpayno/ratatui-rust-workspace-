# [counter-app](https://ratatui.rs/tutorials/counter-app/)

Simple counter app.

## Commands

Use [bacon](https://github.com/Canop/bacon) to watch the project.

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=counter-app-watch promptEnv=true terminalRows=20 }
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

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=counter-app-add-deps promptEnv=true terminalRows=20 }
set -e

stty rows "${LINES:-25}"
stty cols "${COLUMNS:-80}"

printf "PWD: %s\n" "${PWD}"
printf "\n"

: sudo nala install -y --no-autoremove xxx

printf "\n"

cargo add ratatui
cargo add crossterm
cargo add color-eyre
```

Run the project.

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=counter-app-run promptEnv=true terminalRows=20 }
set -e

stty rows "${LINES:-25}"
stty cols "${COLUMNS:-80}"

printf "PWD: %s\n" "${PWD}"
printf "\n"

reset
cargo run
```

