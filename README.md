[![Build Status](https://travis-ci.org/initbar/blaze.svg?branch=master)](https://travis-ci.org/initbar/blaze)
<p align="center">
  <img src="https://raw.githubusercontent.com/initbar/blaze/docs/logo.png">
</p>

**blaze** is a rustic-Python library for blazingly-fast text manipulation at a quantum level.

This library was written since Python is *manageably slow* with string manipulation operations. Since many Python developers desire the performance of low-level languages (e.g. C++) without having to add unnecessary complexity, **blaze** allows the developers to tap into the performance of [rust language](https://www.rust-lang.org) by interfacing between the two different languages.

## Dependencies

- [cargo](https://github.com/rust-lang/cargo)
- [pip](https://github.com/pypa/pip) (optional)
  - pytest
  - pytest-benchmark

## What can it do?

Currently, **blaze** focuses on string manipulations only and makes sure to do them very well:

| methods     | description |
| :---------- |:------------|
| .count()    | *counts the frequency of patterns inside text*. |
| .replace()  | *replaces matching pattern with a replacement string inside text*. |
| .to_lower() | *transforms a text into all uppercase*. |
| .to_upper() | *transforms a text into all lowercase*. |

## Why **blaze**?

Compared to native Python methods, all operations - using the power of [rust](https://www.rust-lang.org) - are at least ~50% faster (and some are approximately 133% faster). For more information, see [benchmark](#benchmark).

## How does **blaze** perform?

| **Test** | **Python** | **Blaze** |
|:--------:|:----------:|:---------:|
| .count() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/count.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/count.png) |
| .replace() (replace) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/replace.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/replace.png) |
| .replace() (regex) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/regex.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/regex.png) |
| .to_lower() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/lowercase.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/lowercase.png) |
| .to_upper() | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/python/uppercase.png) | ![](https://raw.githubusercontent.com/initbar/blaze/docs/benchmark/rust/uppercase.png) |

## Build

- By default, **blaze** is built against the Python 2.7 libraries. In order to build for Python 3, switch to Python 3 features under [Cargo.toml](https://github.com/initbar/blaze/blob/master/src/Cargo.toml). Otherwise, building is as simple as running `make`:

```bash
~$ make
~$ make test # optional
```

- To build using [docker](https://www.docker.com), use the following [make](https://github.com/initbar/blaze/blob/master/Makefile) parameters:
  - make *docker-centos*
  - make *docker-ubuntu*

**Note**: once the build is finished, you'll see a link to https://file.io at the end of [docker log](https://docs.docker.com/engine/reference/commandline/logs) or STDOUT.

## Usage

It is very straightforward to use **blaze**. When [build](#build) is finished, there should be [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) binaries ("*blaze.so*") at the project base and the `tests/` directory.

From your Python codes, simply `import`:

```python
#!/usr/bin/env python
import blaze # this will import "blaze.so"
```

## License

**blaze** is under [MIT License](./LICENSE.md).
