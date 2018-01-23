[![Build Status](https://travis-ci.org/initbar/blaze.svg?branch=master)](https://travis-ci.org/initbar/blaze)
<p align="center">
  <img src="https://raw.githubusercontent.com/initbar/blaze/docs/logo.png">
</p>

**blaze** is a rustic-Python library for blazingly-fast text manipulation at a quantum level.

This library was written since Python is quite "slow" with string manipulation operations. Since many Python developers desire the raw performance of low-level languages (e.g. C++) without adding much complexity, **blaze** was written so that the developers can tap into the string manipulation performance of [rust](https://www.rust-lang.org) language.

## Dependencies

- [cargo](https://github.com/rust-lang/cargo)
- [pip](https://github.com/pypa/pip) (optional)
  - pytest
  - pytest-benchmark

## What can it do?

Currently, **blaze** can do the followings (with more to come with improvements):

- **.count()** - *counts the frequency of patterns inside text*.
- **.replace()** - *replaces matching pattern with a replacement string inside text*.
- **.to_lower()** - *transforms a text into all uppercase*.
- **.to_upper()** - *transforms a text into all lowercase*.

## Build

By default, **blaze** is built against the Python 2.7 libraries. In order to build for Python 3, switch the `cpython` dependencies under [Cargo.toml](https://github.com/initbar/blaze/blob/master/src/Cargo.toml).

Otherwise, build is as simple as running `make`:

```bash
~$ make
~$ make test # optional
```

## Usage

When build is finished, there should be [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) binaries (*blaze.so*) at the project base and the `tests/` directory.

To use the library, simply `import` to any Python projects:

```python
#!/usr/bin/env python
import blaze
```

## License

**blaze** is under [MIT License](./LICENSE.md).
