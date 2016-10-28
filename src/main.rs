pub mod udp_server;
pub mod data_structs;

extern crate csv;
extern crate chrono;
extern crate byteorder;


fn main() {

    // need to get these from command line
    let lines_to_skip: usize = 4;


    udp_server::socket_response("0.0.0.0", 13389, lines_to_skip);
}
