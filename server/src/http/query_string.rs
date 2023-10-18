/*
Implementation of a data structure to represent the query string in an HTTP request.
The string is basically a map from keys to values, but when parsing the string there
are some edge cases to think of when it comes to the values. There could be an empty
value if you have something like '?a=' or '?a', and you can also have multiple values
like '?b=1&b=2'. So we need to handle all these cases.
*/
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

// Represents the value of the query string map
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str), // it's possible that there is a single value...
    Multiple(Vec<&'buf str>), // or multiple values for the same key
}

impl<'buf> QueryString<'buf> {
    /// Get the value from the query string, if it exists
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// for converting the query string string slice to the QueryString object,
// basically initializing the data structure
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        // create empty hash map
        let mut data = HashMap::new();

        // scan over '&key=value' pairs in the query string
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            // determine which chars are the key and value
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[(i+1)..];
            }
            // gets the (key, value) pair (the "entry") from the hash map
            data.entry(key)
            // if it already exists, take the previous value, and put it
            // in a vector with the new value
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(vec) => {vec.push(val)}
            })
            // otherwise, insert just the value itself
            .or_insert(Value::Single(val));
        }
        QueryString {data}
    }
}