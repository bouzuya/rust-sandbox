# twiq-light

## Installation

```console
$ git clone https://github.com/bouzuya/rust-sandbox
# ...

$ cd twiq-light/

$ cargo install --path .
```

## Usage

### Queue

```console
$ twiq-light queue --help
Usage: twiq-light queue <COMMAND>

Commands:
  authorize
  dequeue
  enqueue
  list
  remove
  reorder
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

### Tweet

```console
$ twiq-light tweet import /path/to/exported-data/data/tweet.js

$ # twiq-light queue authorize
$ twiq-light tweet fetch

$ twiq-light tweet search

$ twiq-light tweet --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/twiq-light tweet --help`
Usage: twiq-light tweet <COMMAND>

Commands:
  fetch
  import
  search
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

## Dirs

- `XDG_CONFIG_DIR=${HOME}/.config`
- `XDG_STATE_DIR=${HOME}/.local/state`
- `TWIQ_LIGHT_CONFIG_DIR=${XDG_CONFIG_DIR}/net.bouzuya.rust-sandbox.twiq-light`
- `TWIQ_LIGHT_STATE_DIR=${XDG_STATE_DIR}/net.bouzuya.rust-sandbox.twiq-light`

## Files

- `${TWIQ_LIGHT_CONFIG_DIR}/config.json`
- `${TWIQ_LIGHT_STATE_DIR}/tweet.json`
