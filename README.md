[![Build Status](https://travis-ci.org/initbar/blaze.svg?branch=master)](https://travis-ci.org/initbar/blaze)
<p align="center">
  <img src="https://raw.githubusercontent.com/initbar/blaze/docs/logo.png">
</p>

**blaze** is a rustic-Python library for blazingly-fast text manipulation at a quantum level.

## Dependencies

- [cargo](https://github.com/rust-lang/cargo)
- [pip](https://github.com/pypa/pip) (optional)
  - pytest
  - pytest-benchmark

## Build

By default, **blaze** is built against the Python 2.7 libraries. In order to build for Python 3, switch the `cpython` dependencies under [Cargo.toml](https://github.com/initbar/blaze/blob/master/src/Cargo.toml).

Otherwise, build is as simple as running `make`:

```bash
~$ make
~$ make test # optional
```

## Usage

When build is finished, there should be a `blaze.so` [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) binaries at the project base and the `tests/` directory.

```python
import blaze
print dir(blaze)
```

## License

**blaze** is under [MIT License](./LICENSE.md).
