use std::collections::HashMap;


// struct that holds info for the columns in the parsed file
#[derive(Debug, Clone)]
pub struct CsvRows {
    pub date: String,
    pub time: String,
    pub info: HashMap<String, String>
}