# nostrs

A nostr client written in Rust.

## Usage

```console
$ # login
$ env PRIVATE_KEY=nsecxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx nostrs keypair create
$ cat ~/.local/state/net.bouzuya.rust-sandbox.nostrs/private_key.json
{"private_key":"nsecxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"}

$ # create a new text-note
$ nostrs text-note create 'Hello, nostr!'
note1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```
