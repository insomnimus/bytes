# Bytes
Pretty print byte values.

# Examples

```sh
$ bytes 2040mib
1.9921875GiB
$ bytes --raw 1kb
1000
$ bytes --raw 1kib
1024
$ echo "0.7gib" | bytes
716.7999992370605MiB
$ echo 4096 | bytes
4KiB
```

# Install

```sh
git clone https://github.com/insomnimus/bytes
cd bytes
cargo install --locked --path .
```
