# date-range

```console
$ date-range 2021-01-04 # --format date
2021-01-04/2021-01-04

$ date-range 2021-01 # --format month
2021-01-01/2021-01-31

$ date-range 2021 # --format year
2021-01-01/2021-12-31

$ date-range 2021-W01-1 # --format week-date
2021-01-04/2021-01-04

$ date-range 2021-W01 # --format week
2021-01-04/2021-01-10

$ date-range --format week-year 2021
2021-01-04/2022-01-02

$ date-range 2021-Q1 # --format quarter
2021-01-01/2021-03-31

$ date-range --first 2021-Q1
2021-01-01

$ date-range --last 2021-Q1
2021-03-31
```
