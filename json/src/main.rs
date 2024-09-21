use std::fs;

use serde_json::Value;

fn main() {
    let data = fs::read_to_string("sample.json").expect("read sample.json error");

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data.as_str()).expect("unmarshal sample.json error");

    // Access parts of the data by indexing with square brackets.
    let binding = v["name"].to_string();
    let aa = binding.as_str();

    print!("{}", aa)
}
