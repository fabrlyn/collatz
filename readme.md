# collatz

[Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) crates.

- [cli](./cli/)
- [library](./collatz/)

## development

### prerequisites

- [cargo llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) - test coverage
- [gnuplot](http://www.gnuplot.info/) - plotter for benchmarking
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

### benchmark

**Run benchmark**
```sh
$ cargo bench -- --verbose 
```

**View graphs**
```sh
$ open target/criterion/report/index.html
```

### install

```sh
$ make install
```
