use std::net::UdpSocket;
use std::io::Error;
use std::string::String;
use data_structs::CsvRows;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::env;
use std::cmp::Ordering;

use csv;
use byteorder::{ByteOrder, LittleEndian};
use chrono::*;


fn int_to_char(byte_array: &[u8; 255]) -> String{
    //clean trailing 0s
    let mut new_vec = Vec::new();
    for i in byte_array.iter(){
        if *i != 0 {
            new_vec.push(*i);
        }
    }

    String::from_utf8(new_vec).unwrap()
}

fn time_compare(start_time: &str, end_time: &str, compare_time: &str) -> bool {
    fn parse_date(date: &str) -> Date<UTC>{
        let components: Vec<&str> = date.split("/").collect();
        UTC.ymd(components[2].parse::<i32>().unwrap(), components[0].parse::<u32>().unwrap(), components[1].parse::<u32>().unwrap())
    }

    let start_time = parse_date(start_time);
    let end_time = parse_date(end_time);
    let compare_time = parse_date(compare_time);

    if ((compare_time.cmp(&start_time) == Ordering::Greater) || (compare_time.cmp(&start_time) == Ordering::Equal))
        && ((compare_time.cmp(&end_time) == Ordering::Less) || (compare_time.cmp(&end_time) == Ordering::Equal)){
        true
    }
    else {
        false
    }
}

pub fn socket_response(listen_addr: &str, listen_port: i32, lines_to_skip: usize) -> Result<(), Error> {

    let bind_addr = format!("{}:{}", listen_addr, listen_port);
    let socket = try!(UdpSocket::bind(&bind_addr.as_str()));
    println!("{:?}", socket);

    // device discovery
    let mut media_path = format!("/media/{:?}", env::var("LOGNAME").unwrap()); // get username for media path from "LOGNAME" environment var
    media_path = str::replace(&media_path, "\"", "");
    let paths = fs::read_dir(&media_path).unwrap();

    let mut device_path: String = String::new();
    for path in paths {
        // skip boot partitions
        let foldername = path.unwrap().path().display().to_string();
        if foldername.contains("system-boot"){continue}
        device_path = foldername;
    }
    let device_name = str::replace(&device_path, &format!("{}/", &media_path), "");
    let path_to_csv: &str = &format!("{}/SingleMeasurementLog.csv", device_path);
    println!("{}, {}", device_name, device_path);

    loop {
        // read from the socket
        let mut buf = [0; 255];
        let (amt, src) = try!(socket.recv_from(&mut buf));

        let message = int_to_char(&buf);
        println!("{:?}", &message);
        println!("{:?}", src);

        // send a reply to the socket we received data from
        let buf = &mut buf[..amt];
        buf.reverse();
//        try!(socket.send_to(buf, &src));

        // quit if instructed to do so
        // "&" in front of message converts String type to &str type
        if &message == "QUIT"{
            break;
        }

        else if &message == "PING" {
            let message_str = format!("{}", device_name);
            let message:&[u8] = message_str.as_bytes();
            try!(socket.send_to(message, &src));
        }

        else if&message == "SEND" {
            // receive date info from socket
            let mut buf = [0; 255];
            let (amt, src) = try!(socket.recv_from(&mut buf));
            let date_range = int_to_char(&buf);

            // remove undesired dates
            let split: Vec<&str> = date_range.split("-").collect();
            let start_date = split[0];
            let end_date = split[1];

            // CSV parsing
            let file_str = read_file(path_to_csv).unwrap(); //TODO: should do a match
            let mut csv_file = parse_csv(file_str, lines_to_skip, start_date, end_date);

            let mut send_buf = [0;255];
            let mut size: u32 = csv_file.len() as u32;
            LittleEndian::write_u32(&mut send_buf, size);

            try!(socket.send_to(&send_buf, src));

            while !csv_file.is_empty() {
                let mut len_of_message = csv_file.len();
                if len_of_message >= 255{
                    len_of_message = 255
                }

                let mut send_buf: Vec<u8> = csv_file.drain(0..len_of_message).collect();
                try!(socket.send_to(&send_buf, src));
            }

//            FOR EMERGENCIES ONLY
//            try!(socket.send_to(b"STOP", src));
        }
    }
    Ok(())
}   // the socket is closed here



pub fn socket_send() -> Result<(), Error> {

    let socket = try!(UdpSocket::bind("0.0.0.0:0"));
    try!(socket.set_broadcast(true));
    println!("{:?}", socket);

    // put message here
    let array:&[u8] = b"QUIT";
    try!(socket.send_to(&array, "255.255.255.255:13389"));

    let mut buf = [0; 255];
    let (amt, src) = try!(socket.recv_from(&mut buf));
    println!("{:?}",amt);
    //println!("{:?}", buf);
    println!("{:?}", src);

    Ok(())

}   // the socket is closed here


fn read_file(path_to_file: &str)-> Result<String, io::Error> {
    let mut f = try!(File::open(path_to_file));
    let mut buffer = String::new();

    try!(f.read_to_string(&mut buffer));
    Ok(buffer)
}


fn parse_csv(file_str: String, lines_to_skip: usize, start_date: &str, end_date: &str) -> Vec<u8> {

    let mut reader = csv::Reader::from_string(file_str).delimiter(b';').flexible(true);
    let mut writer = csv::Writer::from_memory();

    // setup vars
    let header_line_index = lines_to_skip + 1;
    let mut header_strings: Vec<String> = Vec::new();

    for (i, row) in reader.decode().enumerate() {
        let mut csv_details = Vec::new();
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

        let mut row_vec: Vec<String> = row.unwrap();

        // we get all rows, so we need to split into individual cells here
        for (x, item) in row_vec.iter().enumerate() {

            let cell = str::replace(item, "\u{0}", ""); // a lot of weird data here
            if cell == "" {
                continue;
            }

            // construct header information
            if i == header_line_index {
                csv_details.push(cell.clone());
            }
            else {
                if x == 0 { //date
                    let date_in_range = time_compare(&start_date, &end_date, &cell);
                    if !date_in_range{continue;}
                    csv_details.push(cell.clone());
                    row_info.date = cell.clone();
                }
                    else if x == 1 { //time
                        csv_details.push(cell.clone());
                        row_info.time = cell.clone();
                    }
                        else if x == 2 { // this is the test number if ever needed
                            //                    println!("Test number {}", cell);
                        }
                            else { //other info
                                csv_details.push(cell.clone());
//                                csv_info.insert(header_strings[x-3].clone(), cell);
                            }
            }
        }
        if row_info.date != "" { // need to skip the first iteration where header info is built
            // run info
            row_info.info = csv_info;

            // populate vec with updated info
            writer.encode(csv_details);
        }
        else{
            let mut date_header = vec!("Date".to_string(), "Time".to_string());
            date_header.append(&mut csv_details);
            writer.encode(date_header);
        }
    }

    writer.as_bytes().to_vec()
}