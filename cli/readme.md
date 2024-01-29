# collatz

[Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) cli crate.

## usage

**print sequence**

```sh
$ collatz sequence 2
4
2
1
```

**print enumerated sequence**

```sh
$ collatz sequence --enumerate 2
0: 4
1: 2
2: 1
```

**print step count**

```sh
$ collatz count 2
2
```

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

