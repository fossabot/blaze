[![Build Status](https://travis-ci.org/initbar/blaze.svg?branch=master)](https://travis-ci.org/initbar/blaze)
<p align="center">
  <img src="https://raw.githubusercontent.com/initbar/blaze/docs/logo.png">
</p>

**blaze** is a [rust](https://www.rust-lang.org)-Python module to make Python codes blazingly-fast.

## What can it do?

This module was written in order to extend Python codes to use the performance of low-level languages with zero complexity. **blaze**, in essence, allows Python developers to tap into the performance of [rust language](https://www.rust-lang.org) by interfacing between the two languages.

For specific code details, please see [developer documentation](#).

## Why **blaze**?

Compared to native Python methods, **blaze** outperforms by ~50% (and some are approximately 133% faster).

| **Test** | **Python** | **Blaze** |
|:--------:|:----------:|:---------:|
| .count() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/count.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/count.png) |
| .replace() (replace) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/replace.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/replace.png) |
| .replace() (regex) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/regex.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/regex.png) |
| .to_lower() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/lowercase.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/lowercase.png) |
| .to_upper() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/uppercase.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/uppercase.png) |

## Build

There is only [cargo](https://github.com/rust-lang/cargo) dependency to compile the rust code into Python module.

```bash
~$ sudo apt install cargo
```

By default, **blaze** is built against the Python 2.7 libraries for maximum platform support. However, in order to build for Python 3+, please [edit Cargo.toml](./src/Cargo.toml) and run `make`. Otherwise, building is as simple as running `make`:

```bash
~$ make
~$ # make release
```

## Tests

Running benchmarks and tests requires some [pip](https://github.com/pypa/pip) packages.

```bash
~$ # sudo -H pip install pytest pytest-benchmark
~$ make test
```

## Usage

It is very straightforward to use **blaze**. When [build](#build) is completed, a binary called "*blaze.so*" will be generated at the project base.

From there, in your Python code, just use the `import` keyword:

```python
import blaze # imports "blaze.so"
```

## Docker

To prevent tainting local environment, build using [docker](https://www.docker.com) is recommended. Use the [make](https://github.com/initbar/blaze/blob/master/Makefile) commands and you'll see a https://file.io link at the end of [docker log](https://docs.docker.com/engine/reference/commandline/logs) or STDOUT).

```bash
~$ make docker-centos # centos 6.7
~$ make docker-ubuntu # ubuntu 16.04
```

## License

**blaze** is under [MIT License](./LICENSE.md).
