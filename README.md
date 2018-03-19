[![Build Status](https://travis-ci.org/initbar/blaze.svg?branch=master)](https://travis-ci.org/initbar/blaze)
<p align="center">
  <img src="https://raw.githubusercontent.com/initbar/blaze/docs/logo.png">
</p>

**blaze** is a [rust](https://www.rust-lang.org)-Python library for blazingly-fast text manipulation at a quantum level.

This library was written since Python is *manageably slow* with string manipulation operations. Since many Python developers desire the performance of low-level languages (e.g. C++) without having to add unnecessary complexity, **blaze** allows the developers to tap into the performance of [rust language](https://www.rust-lang.org) by interfacing between the two different languages.

## What can it do?

Please see [developer documentation]().

## Why **blaze**?

Compared to native Python methods, **blaze** outperforms by ~50% (and some are approximately 133% faster). For some benchmarks:

| **Test** | **Python** | **Blaze** |
|:--------:|:----------:|:---------:|
| .count() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/count.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/count.png) |
| .replace() (replace) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/replace.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/replace.png) |
| .replace() (regex) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/regex.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/regex.png) |
| .to_lower() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/lowercase.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/lowercase.png) |
| .to_upper() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/uppercase.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/uppercase.png) |

## Build

There is only [cargo](https://github.com/rust-lang/cargo) dependency to compile the rust code into Python module. However, if you want to run unit tests and benchmarks, you can also install some [pip](https://github.com/pypa/pip) packages.

```bash
~$ sudo apt install cargo
~$ sudo -H pip install pytest pytest-benchmark
```

By default, **blaze** is built against the Python 2.7 libraries for maximum platform support. However, in order to build for Python 3+, edit [Cargo.toml](./src/Cargo.toml). Otherwise, building is as simple as running `make`:

```bash
~$ make
~$ make test # optional
```

## Usage

It is very straightforward to use **blaze**. When [build](#build) is finished, there should be an [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) binary ("*blaze.so*") at the project base and the `tests/` directory.

In your Python code, just use the `import` keyword:

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
