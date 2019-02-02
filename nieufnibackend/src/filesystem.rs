use std::fs::{ read_to_string };

pub fn index_file() -> String {
    read_to_string("../nieufnifront/dev/index.html").expect("index.html is not present")
}
