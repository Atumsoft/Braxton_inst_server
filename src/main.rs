pub mod udp_server;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

extern crate simple_csv;
use simple_csv::*;


fn read_file(path_to_file: &str)-> Result<String, io::Error> {
    let mut f = try!(File::open(path_to_file));
    let mut buffer = String::new();

    try!(f.read_to_string(&mut buffer));
    Ok(buffer)
}

// struct that holds info for the columns in the parsed file
#[derive(Debug)]
struct CsvRows {
    date: String,
    time: String,
    info: HashMap<String, String>
}


fn parse_csv(file_str: String, lines_to_skip: usize) -> Vec<CsvRows> {
    // setup csv reader
    let bytes = file_str.into_bytes();
    let csv_reader = &*bytes;

    let mut csv_options: SimpleCsvReaderOptions = Default::default();
    csv_options.delimiter = ';';
    let reader = SimpleCsvReader::new(csv_reader);

    // setup vars
    let header_line_index = lines_to_skip + 1;
    let mut header_strings: Vec<String> = Vec::new();
    let mut csv_details = Vec::<CsvRows>::new();

    for (i, row) in reader.enumerate() {
        let mut csv_info = HashMap::<String, String>::new();
        let mut row_info = CsvRows {date : "".to_string(), time: "".to_string(), info: HashMap::<String, String>::new()};

        // need to skip lines for header info depending on instrument
        if i <= lines_to_skip {
            continue;
        }

        // skip headers on every other line
        if (i > header_line_index) && (i - (header_line_index)) % 2 == 0 {
            continue;
        }

        let ref mut row_vec = row.unwrap()[0];

        // we get all rows, so we need to split into individual cells here
        for (x, item) in row_vec.split(";").enumerate() {

            let cell = str::replace(item, "\u{0}", ""); // a lot of weird data here
            if cell == "" {
                continue;
            }

            // construct header information
            if i == header_line_index {
                header_strings.push(cell.clone());
            }
            else {
                if x == 0 { //date
                    row_info.date = cell.clone();
                }
                else if x == 1 { //time
                    row_info.time = cell.clone();
                }
                else if x == 2 { // this is the test number if ever needed
//                    println!("Test number {}", cell);
                }
                else { //other info
                    csv_info.insert(header_strings[x-3].clone(), cell);
                }
            }
        }
        if row_info.date != "" { // need to skip the first iteration where header info is built
            // run info
            row_info.info = csv_info;

            // populate vec with updated info
            csv_details.push(row_info);
        }
    }

    csv_details
}


fn main() {
    // need to get these from command line
    let lines_to_skip: usize = 6;
    let path_to_csv: &str = "/home/andy/Documents/SingleMeasurementLog.csv";

    let file_str = read_file(path_to_csv).unwrap(); //TODO: should do a match
    let csv_details = parse_csv(file_str, lines_to_skip);

    for test in csv_details {
        println!("{:?}", test);
    }

    udp_server::socket_response("0.0.0.0", 13389);
}
