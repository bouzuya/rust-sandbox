# date-to-week-date

## Usage

### Usage (Docker)

```console
$ git clone https://github.com/bouzuya/rust-sandbox
# ...

$ cd rust-sandbox/date-to-week-date/

$ docker build -t date-to-week-date .

$ docker run --detach --publish 8080:8080 --name date-to-week-date --rm date-to-week-date

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
