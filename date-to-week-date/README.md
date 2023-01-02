# date-to-week-date

## Usage

```console
$ git clone https://github.com/bouzuya/rust-sandbox
# ...

$ cd rust-sandbox/date-to-week-date/

$ cargo run
# ...

$ # in other terminal
$ curl -s 'http://localhost:8080/healthz'
OK

$ curl -s 'http://localhost:8080/?date=2023-01-01'
2022-W52-7

$ curl -s 'http://localhost:8080/?date=2023-01-02'
2023-W01-1
```

```console
$ env PORT=3000 cargo run
# ...

$ # in other terminal
$ curl -s 'http://localhost:3000/?date=2023-01-02'
2023-W01-1

```
