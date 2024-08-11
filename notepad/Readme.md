# Simple Notepad

## Kullanım

```shell
cargo run -- --help

Usage: notepad [ACTION]

Arguments:
  [ACTION]  

Options:
  -h, --help     Print help
  -V, --version  Print version
```

`Action`, hangi işlemin uygulanacağını belirtmenizi sağlar. Bunlar: `add, get, count, edit ve remove`.

### Not Ekleme

```
cargo run -- add
Enter your note: lorem ipsum dolor sit amet.
```

## test
```shell
cargo test
```