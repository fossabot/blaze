#[macro_use]
extern crate cpython;
use cpython::{Python, PyResult};

fn rust_print(_py: Python, text: &str) -> PyResult<u64> {
    println!("{}", text);
    Ok(0)
}

py_module_initializer!(libblazelib, initlibblazelib, PyInit_blazelib, |py, m| {
    try!(m.add(py, "__doc__", "Blaze string manipulation library in Rust."));
    try!(m.add(py, "rust_print", py_fn!(py, rust_print(text: &str))));
    Ok(())
});
