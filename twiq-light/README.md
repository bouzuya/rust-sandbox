# twiq-light

```console
$ cargo run -- import /path/to/exported-data/data/tweet.js
$ cargo run -- fetch
# ...

$ cat ~/twiq-light.json | jq -r '[.[]] | sort_by(.at) | .[] | .at + " " + .text + "\n"'
```
