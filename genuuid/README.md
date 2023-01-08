# genuuid

**You can use `uuidgen` instead of genuuid**.

An online UUID generator.

## Usage

```console
$ # If you use `bouzuya.net`:

$ curl -Ls 'https://bouzuya.net/lab/genuuid'
186a7b6c-95de-4183-94b9-5c4d5df51657⏎
```

```console
$ # If you use CLI:

$ genuuid generate
9af362e0-ea35-4a6d-8ac9-2e81e4897ff8

$ # show help
$ genuuid help
# ...

$ # run server
$ genuuid server

$ curl -L 'http://localhost:8080'
207db46e-c6ac-47f6-b6a3-de81486af44f

$ curl -L 'http://localhost:8080/uuids.txt?count=2'
207db46e-c6ac-47f6-b6a3-de81486af44f
6b0822f5-2326-4462-a043-f330d46fa09c
```

## Installation

```console
$ If you use `cargo`:

$ git clone https://github.com/bouzuya/rust-sandbox.git
$ cd rust-sandbox/genuuid/
$ cargo install --path .
# ...

$ If you use `docker`:

$ docker run ghcr.io/bouzuya/rust-sandbox/genuuid:b7e9834af55ccf928c8d4c4c87403bed8530258b
```
