# genuuid

## Usage

```console
$ curl -L 'https://bouzuya.net/lab/genuuid'
186a7b6c-95de-4183-94b9-5c4d5df51657⏎
```

```console
$ # run local

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
$ cargo install --path .
# ...
```
