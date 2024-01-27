# collatz

Library and command line application for [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture).

## development

### prerequisites

- [cargo llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) - test coverage
- [upx](https://github.com/upx/upx) - binary packer

### test

```sh
$ cargo test
```

### coverage

**coverage overview**

```sh
$ cargo llvm-cov test
```

**coverage details**

```sh
$ cargo llvm-cov test --open
```

### install

```sh
$ make install
```

