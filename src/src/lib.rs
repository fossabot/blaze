#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn stub(_py: Python, text: &str) -> PyResult<i32> {
    return Ok(0);
}

py_module_initializer!(libblaze, initlibblaze, PyInit_blaze, | py, m | {
    try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    try!(m.add(py, "stub", py_fn!(py, stub(val: &str))));
    Ok(())
});
