# nostr-keygen

nostr-keygen は prefix を指定して nostr で使用するための keypair を生成するコマンドである。

prefix は public key の NIP-19 <https://github.com/nostr-protocol/nips/blob/master/19.md> で指定されている bech32 形式から npub1 を除いたものである。たとえば `60` を prefix に指定すると `npub160ulkrt40kpqtr6l3jd20tcswuykx2wkfwn6pskn8d8rp0c3dznsvd7tm4` などがマッチする。

注意事項として bech32 形式 (BIP-0173 <https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki>) における prefix であるため、 `'1'`, `'b'`, `'i'`, `'o'` は指定できない。これは指定しても永久にマッチしないためである。

## Usage

```console
$ # install `nostr-keygen` command
$ cargo install --path .
...

$ # run `nostr-keygen` with `rs` prefix
$ nostr-keygen rs
public_key  = npub1rslx02hnntqpst3npdvyv0wag93rep7nxgpcfc227257j4t3ulms63k38j
private_key = nsec1mt6tg4602wrqkm2hdt52ferzcp38m9fakp8kjmj09mgwt4yv6usscptjpm
```

## Examples

```console
$ time nostr-keygen 60
public_key  = npub160ulkrt40kpqtr6l3jd20tcswuykx2wkfwn6pskn8d8rp0c3dznsvd7tm4
private_key = nsec1dztfzv74we2m29jeh2a4qd7u3u5hqh02wf3lp63nytda4gey2uss7lemvk
nostr-keygen 60  0.61s user 0.01s system 64% cpu 0.957 total

$ time nostr-keygen 60u
10000
20000
30000
public_key  = npub160u6r7se9xtyar83jemzhkr6xmk7pp8fhj7jd0nct0msy0kj7zsq5c6rjx
private_key = nsec187qlk6a0v5t37d2v8r0u5dfn2gx9tl7mnrmvz42xh9vus7z336nqxwa8lh
nostr-keygen 60u  6.43s user 0.06s system 97% cpu 6.635 total

$ time nostr-keygen 60uz
10000
20000
...
2510000
public_key  = npub160uzj7q9rk0ul865gs2ftu6pu4rh67rt4yejpgp6fzuerz90m6kqkdl7gz
private_key = nsec1wl5sr2pa3d9ewh0s3as9w3ln8t39g8d4ql92x6nn9dwr224f49psuflmps
nostr-keygen 60uz  463.28s user 5.17s system 95% cpu 8:11.42 total
```
