# twiq-light

## Installation

```console
$ cargo install --path .
```

## Usage

```console
# env GOOGLE_APPLICATION_CREDENTIALS=...
# env PROJECT_ID=...
$ twiq-light tweet import /path/to/exported-data/data/tweet.js

$ twiq-light tweet fetch

$ twiq-light tweet search

$ twiq-light help
Usage: twiq-light <COMMAND>

Commands:
  queue
  tweet
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

$ Usage: twiq-light tweet <COMMAND>

Commands:
  fetch
  import
  search
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```
