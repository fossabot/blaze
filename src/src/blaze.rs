#[macro_use] extern crate cpython;
use cpython::{Python, PyResult, PyObject};

py_module_initializer!(blaze, initblaze, PyInit_blaze, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "stub", py_fn!(py, stub()))?;
    Ok(())
});

fn stub(py: Python) -> PyResult<PyObject> {
    Ok(py.None())
}
