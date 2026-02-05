# Netero

Command-line assistant written in Rust.

This is an experimental project created while learning Rust.
Its purpose is to explore basic integration with large language models (LLMs)
from a terminal environment.

The interface, behavior, and internal structure are incomplete and subject to change.


## Usage

CLI for interacting with language models.

```
Usage: netero [OPTIONS] [PROMPT] [COMMAND]
```

### Commands

* `commit`
  Generate a commit message

* `prompt`
  Process a single prompt

### Arguments

* `[PROMPT]`
  Prompt passed to the language model

### Options

* `-p, --provider <PROVIDER>`
  Language model provider (default: `codestral`)

* `-v, --verbose`
  Enable verbose output

* `-h, --help`
  Print help

* `-V, --version`
  Print version
