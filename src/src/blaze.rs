#[macro_use]

extern crate cpython;

use cpython::{Python, PyResult, PyObject};

//
// initialization
//

py_module_initializer!(blaze, initblaze, PyInit_blaze, |py, m| {
    m.add(py, "__doc__", "blazingly-fast text manipulation engine at a quantum level.")?;
    m.add(py, "__name__", "blaze")?;
    m.add(py, "__version__", "0.1.0")?;
    m.add(py, "replace", py_fn!(py, replace(text: &str,
                                            repl: &str,
                                            text: &str)))?;
    m.add(py, "to_lower_case", py_fn!(py, to_lower_case(text: &str)))?;
    m.add(py, "to_upper_case", py_fn!(py, to_upper_case(text: &str)))?;
    Ok(())
});

//
// methods
//

/* replace(pattern, repl, text) -- search and replace a `pattern` with `repl`
 *                                 inside given text.
 * + {str} pattern -- absolute string.
 * + {str} repl -- replacement string.
 * + {str} text -- input string.
 */
fn replace(py: Python,
           pattern: &str,
           repl: &str,
           text: &str) -> PyResult<PyObject> {
    Ok(py.None())
}

/* to_lower_case(text) -- transform text to lowercase.
 * + {str} text -- input string.
 */
fn to_lower_case(py: Python,
                 text: &str) -> PyResult<PyObject> {
    Ok(py.None())
}

/* to_upper_case(text) -- transform text to lowercase.
 * + {str} text -- input string.
 */
fn to_upper_case(py: Python,
                 text: &str) -> PyResult<PyObject> {
    Ok(py.None())
}
