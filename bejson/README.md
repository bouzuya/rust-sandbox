# bejson

A JSON generator that can embed the results of command execution.

## Usage

```console
$ cat test.bejson
{
    "foo": "bar",
    "greeting": $`echo 'Hello'`,
    "date": $`date "+%Y-%m-%d" | awk '{ printf $0 }'`
}

$ bejson test.bejson
{"foo":"bar","greeting":"Hello\n","date":"2021-05-28"}
```

## Installation

```console
$ version=0.2.0
$ curl -L "https://github.com/bouzuya/rust-sandbox/releases/download/bejson%2F${version}/bejson-x86_64-apple-darwin" > bejson
$ chmod +x bejson
$ ./bejson --help
bejson 0.2.0

USAGE:
    bejson <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>
```
