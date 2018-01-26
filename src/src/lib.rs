#[macro_use]

extern crate cpython;
extern crate bio;

use std::collections::BTreeMap;
use bio::pattern_matching::horspool::Horspool;
use cpython::{
    ObjectProtocol,
    PyDict,
    PyObject,
    PyResult,
    Python,
};

#[allow(unused_doc_comment)]
pub fn count<'a>(_py: Python,
                 text: &'a str,
                 pattern: &'a str)
                 -> PyResult<usize> {
    /** : count the frequency of `pattern` inside `text`.
     * + {&str} text -- input string.
     * + {&str} pattern -- pattern to be counted.
     */
    let horspool = Horspool::new(pattern.as_bytes());
    let vector: Vec<usize> = horspool.find_all(text.as_bytes()).collect();
    return Ok(vector.len());
}

#[allow(unused_doc_comment)]
pub fn to_lower<'a>(_py: Python,
                    text: &'a str)
                    -> PyResult<String> {
    /** : transform `text` to lowercase.
     * + {&str} text -- input string.
     */
    let _text = text.to_string().to_lowercase(); // rust heap transformation
    return Ok(_text);
}

#[allow(unused_doc_comment)]
pub fn to_upper<'a>(_py: Python,
                    text: &'a str)
                    -> PyResult<String> {
    /** : transform `text` to uppercase.
     * + {&str} text -- input string.
     */
    let _text = text.to_string().to_uppercase(); // rust heap transformation
    return Ok(_text);
}

#[allow(unused_doc_comment)]
pub fn unique<'a>(_py: Python,
                  text: &'a str)
                  -> PyResult<String> {
    /** : return distinct text characters.
     * + {&str} text -- input string.
     */
    let mut btree = BTreeMap::new();
    for character in text.chars() {
        if !btree.contains_key(&character) {
            btree.insert(character, ());
        }
    }
    return Ok(btree.keys().collect());
}

#[allow(unused_doc_comment)]
pub fn replace<'a>(_py: Python,
                   text: &'a str,
                   pattern: &'a str,
                   repl: &'a str)
                   -> PyResult<String> {
    /** : search and replace `pattern` with `repl` inside `text`.
     * + {&str} text -- input string.
     * + {&str} pattern -- absolute string.
     * + {&str} repl -- replacement string.
     */
    let _text = text.to_string().replace(pattern, repl);
    return Ok(_text);
}

#[allow(unused_doc_comment)]
pub fn replacen<'a>(_py: Python,
                    text: &'a str,
                    patterns: PyDict)
                    -> PyResult<String> {
    /** : search and replace multiple `patterns`.
     * + {&str} text -- input string.
     * + {PyDict} patterns -- dictionary of replacements.
     */
    let mut _text = text.to_string();
    let entries: Vec<(PyObject, PyObject)> = patterns.items(_py);
    for entry in &entries {
        let (ref key, ref value) = *entry;

        // convert key to &str
        let _key = key.str(_py).unwrap();
        let _key_ref = _key.to_string_lossy(_py);
        let _key_str = _key_ref.as_ref();

        // convert value to &str
        let _value = value.str(_py).unwrap();
        let _value_ref = _value.to_string_lossy(_py);
        let _value_str = _value_ref.as_ref();

        // replace string
        _text = _text.replace(_key_str, _value_str);
    }
    return Ok(_text);
}

//
// initialization
//

py_module_initializer!(blaze, initblaze, PyInit_blaze, |py, m| {
    m.add(py, "__name__", "blaze")?;
    m.add(py, "__doc__", "blazingly-fast text manipulation engine for Python.")?;
    m.add(py, "count", py_fn!(py, count(text: &str, pattern: &str)))?;
    m.add(py, "replace", py_fn!(py, replace(text: &str, pattern: &str, repl: &str)))?;
    m.add(py, "replacen", py_fn!(py, replacen(text: &str, patterns: PyDict)))?;
    m.add(py, "to_lower", py_fn!(py, to_lower(text: &str)))?;
    m.add(py, "to_upper", py_fn!(py, to_upper(text: &str)))?;
    m.add(py, "unique", py_fn!(py, unique(text: &str)))?;
    Ok(())
});
