# markdown-link-helper

## Usage

```console
$ cat test.md
[2021-01-02]

$ cat rule.json
[["^(\\d{4})-(\\d{2})-(\\d{2})$","[$1-$2-$3]: https://blog.bouzuya.net/$1/$2/$3/"]]

$ markdown-link-helper --rule-file rule.json test.md
[2021-01-02]: https://blog.bouzuya.net/2021/01/02/

$ markdown-link-helper --help
Usage: markdown-link-helper --rule-file <RULE_FILE> <FILE>

Arguments:
  <FILE>  The markdown file

Options:
      --rule-file <RULE_FILE>  The rule file
  -h, --help                   Print help
  -V, --version                Print version
```
