# DAS Serialization Reference

DAS choose [Molecule][1] as data serialization standard, this serialization system is maintaining by [Nervos](https://nervos.org).


## Development

### Setup environment

The only thing need here is installing [Molecule][1] and language plugin:

```shell
cargo install moleculec moleculec-go
```

For more details please read [Molecule][1].

### Generate schema codes

Simply run `sh compile.sh <language>`. Currently the following language is supported:

- Rust;
- Go;

> Language name should be lower case in CLI. If your language is not yet supported here, feel free to submit a PR. 😅



[1]: https://github.com/nervosnetwork/molecule
