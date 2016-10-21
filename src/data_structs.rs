use std::collections::HashMap;
use rustc_serialize::Encodable;


// struct that holds info for the columns in the parsed file
#[derive(Debug, Clone, RustcEncodable)]
pub struct CsvRows {
    pub date: String,
    pub time: String,
    pub info: HashMap<String, String>
}