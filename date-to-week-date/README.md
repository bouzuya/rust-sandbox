# date-to-week-date

A Web API server that converts from calendar date (YYYY-MM-DD) to week date (YYYY-Www-D).

## Usage

### Usage (Docker)

```console
$ # See: <https://github.com/bouzuya/rust-sandbox/pkgs/container/rust-sandbox%2Fdate-to-week-date>
$ docker run \
    --detach \
    --publish 8080:8080 \
    --name date-to-week-date \
    --rm \
    ghcr.io/bouzuya/rust-sandbox/date-to-week-date:93c400b3a0d94be81d559726288434c29ef6ad0f
# ...

$ curl -s 'http://localhost:8080/healthz'
OK

$ curl -s 'http://localhost:8080/?date=2023-01-01'
2022-W52-7

$ curl -s 'http://localhost:8080/?date=2023-01-02'
2023-W01-1

$ docker container stop date-to-week-date
```

### Usage (cargo)

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

### `PORT` env

```console
$ env PORT=3000 cargo run
# ...

$ # in other terminal
$ curl -s 'http://localhost:3000/?date=2023-01-02'
2023-W01-1

```

### `BASE_PATH` env

```console
$ env BASE_PATH=/date-to-week-date cargo run
# ...

$ # in other terminal
$ curl -s 'http://localhost:8080/date-to-week-date?date=2023-01-02'
2023-W01-1

```
