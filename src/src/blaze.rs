#[macro_use]

extern crate cpython;

use cpython::{Python, PyResult, PyObject};

/** replace(pattern, repl, text) -- search and replace a `pattern` with `repl`
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

/** to_lower(text) -- transform text to lowercase.
 * + {str} text -- input string.
 */
fn to_lower(py: Python,
            text: &str) -> PyResult<PyObject> {
    Ok(py.None())
}

/** to_upper(text) -- transform text to uppercase.
 * + {str} text -- input string.
 */
fn to_upper(py: Python,
            text: &str) -> PyResult<PyObject> {
    Ok(py.None())
}

//
// initialization
//

py_module_initializer!(blaze, initblaze, PyInit_blaze, |py, m| {
    m.add(py, "__doc__", "blazingly-fast text manipulation engine at a quantum level.")?;
    m.add(py, "__name__", "blaze")?;
    m.add(py, "replace", py_fn!(py, replace(text: &str,
                                            repl: &str,
                                            text: &str)))?;
    m.add(py, "to_lower", py_fn!(py, to_lower(text: &str)))?;
    m.add(py, "to_upper", py_fn!(py, to_upper(text: &str)))?;
    Ok(())
});
