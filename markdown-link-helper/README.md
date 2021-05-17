# markdown-link-helper

## Usage

```console
$ cat test.md
[2021-01-02]

$ cat test.json
[["^(\\d{4})-(\\d{2})-(\\d{2})$","[$1-$2-$3]: https://blog.bouzuya.net/$1/$2/$3/"]]

$ markdown-link-helper test.md --rule-file test.json
[2021-01-02]: https://blog.bouzuya.net/2021/01/02/

$ markdown-link-helper --help
markdown-link-helper 0.2.0
markdown link helper

USAGE:
    markdown-link-helper <FILE> --rule-file <rule-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --rule-file <rule-file>    The rule file

ARGS:
    <FILE>    The markdown file
```
