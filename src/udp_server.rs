use std::net::UdpSocket;
use std::io::Error;
use std::string::String;


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

pub fn socket_response(listen_addr: &str, listen_port: i32) -> Result<(), Error> {

    let bind_addr = format!("{}:{}", listen_addr, listen_port);
    let socket = try!(UdpSocket::bind(&bind_addr.as_str()));
    println!("{:?}", socket);

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
            let message:&[u8] = b"test kev";
            try!(socket.send_to(message, &src));
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



//fn main(){
//    socket_send();
//}
//
//
//fn main(){
//    socket_response("0.0.0.0", 13389);
//}