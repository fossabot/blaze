#[macro_use]

extern crate cpython;
extern crate bio;

use cpython::ObjectProtocol;

pub fn count<'t>(_py :cpython::Python,
                 text: &'t str,
                 pattern: &'t str)
                 -> cpython::PyResult<usize> {
    /** : count the frequency of `pattern` inside `text`.
     * + {&str} text -- input string.
     * + {&str} pattern -- pattern to be counted.
     */
    let horspool = bio::pattern_matching::horspool::Horspool::new(pattern.as_bytes());
    let vector: Vec<usize> = horspool.find_all(text.as_bytes()).collect();
    return Ok(vector.len());
}

pub fn to_lower<'t>(_py :cpython::Python,
                    text: &'t str)
                    -> cpython::PyResult<String> {
    /** : transform `text` to lowercase.
     * + {&str} text -- input string.
     */
    let _text = text.to_string().to_lowercase(); // rust heap transformation
    return Ok(_text);
}

pub fn to_upper<'t>(_py :cpython::Python,
                    text: &'t str)
                    -> cpython::PyResult<String> {
    /** : transform `text` to uppercase.
     * + {&str} text -- input string.
     */
    let _text = text.to_string().to_uppercase(); // rust heap transformation
    return Ok(_text);
}

pub fn unique(_py :cpython::Python,
              text: &str)
              -> cpython::PyResult<String> {
    /** : return distinct text characters.
     * + {&str} text -- input string.
     */
    let mut btree = std::collections::BTreeMap::new();
    for character in text.chars() {
        if !btree.contains_key(&character) {
            btree.insert(character, ());
        }
    }
    return Ok(btree.keys().collect());
}

pub fn replace<'t>(_py :cpython::Python,
                   text: &str,
                   pattern: &str,
                   repl: &str)
                   -> cpython::PyResult<String> {
    /** : search and replace `pattern` with `repl` inside `text`.
     * + {&str} text -- input string.
     * + {&str} pattern -- absolute string.
     * + {&str} repl -- replacement string.
     */
    let _text = text.to_string().replace(pattern, repl);
    return Ok(_text);
}

pub fn replacen<'t>(_py :cpython::Python,
                    text: &'t str,
                    patterns: cpython::PyDict)
                    -> cpython::PyResult<String> {
    /** : search and replace multiple `patterns`.
     * + {&str} text -- input string.
     * + {PyDict} patterns -- dictionary of replacements.
     */
    let mut _text = text.to_string();
    let entries: Vec<(cpython::PyObject, cpython::PyObject)> = patterns.items(_py);
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
    m.add(py, "replacen", py_fn!(py, replacen(text: &str, patterns: cpython::PyDict)))?;
    m.add(py, "to_lower", py_fn!(py, to_lower(text: &str)))?;
    m.add(py, "to_upper", py_fn!(py, to_upper(text: &str)))?;
    m.add(py, "unique", py_fn!(py, unique(text: &str)))?;
    Ok(())
});
