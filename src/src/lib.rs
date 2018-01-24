#[macro_use]

extern crate cpython;
extern crate bio;

use bio::pattern_matching::horspool::Horspool;
use cpython::{Python, PyResult, PyObject};

//
// public
//

/** count(text, character)
 * : count the frequency of `character` inside `text`.
 * + {str} text -- input string.
 * + {str} character -- character to be counted.
 */
#[allow(unused_variables)]
pub fn count<'t>(py: Python,
                 text: &'t str,
                 pattern: &'t str) -> PyResult<usize> {
    let horspool = Horspool::new(pattern.as_bytes());
    let vector: Vec<usize> = horspool.find_all(text.as_bytes()).collect();
    return Ok(vector.len());
}

/** replace(pattern, repl, text)
 * : search and replace `pattern` with `repl` inside `text`.
 * + {str} pattern -- absolute string.
 * + {str} repl -- replacement string.
 * + {str} text -- input string.
 */
pub fn replace<'t>(py: Python,
                   pattern: &'t str,
                   repl: &'t str,
                   text: &'t str) -> PyResult<String> {
    let mut ret = String::new();
    for character in text.chars() {
        match character {
            pattern => ret.push_str(repl),
            _ => ret.push(character)
        }
    }
    return Ok(ret)
}

/** to_lower(text)
 * : transform `text` to lowercase.
 * + {str} text -- input string.
 */
#[allow(unused_variables)]
pub fn to_lower<'t>(py: Python, text: &'t str) -> PyResult<String> {
    let _text = text.to_string().to_lowercase();
    return Ok(_text)
}

/** to_upper(text)
 * : transform `text` to uppercase.
 * + {str} text -- input string.
 */
#[allow(unused_variables)]
pub fn to_upper<'t>(py: Python, text: &'t str) -> PyResult<String> {
    let _text = text.to_string().to_uppercase();
    return Ok(_text)
}

//
// initialization
//

py_module_initializer!(blaze, initblaze, PyInit_blaze, |py, m| {
    m.add(py, "__name__", "blaze")?;
    m.add(py, "__doc__", "blazingly-fast text manipulation engine at a quantum level.")?;
    m.add(py, "count", py_fn!(py, count(text: &str, pattern: &str)))?;
    m.add(py, "replace", py_fn!(py, replace(pattern: &str,
                                            repl: &str,
                                            text: &str)))?;
    m.add(py, "to_lower", py_fn!(py, to_lower(text: &str)))?;
    m.add(py, "to_upper", py_fn!(py, to_upper(text: &str)))?;
    Ok(())
});
