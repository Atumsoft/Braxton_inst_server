use std::net::UdpSocket;
use std::io::Error;
use std::iter::FromIterator;
use std::string::String;


fn int_to_char(byteArray: &[u8; 255]) -> String{
    //clean trailing 0s
    let mut newVec = Vec::new();
    for i in byteArray.iter(){
        if *i != 0 {
            newVec.push(*i);
        }
    }

    String::from_utf8(newVec).unwrap()
}

pub fn socket_response(listenAddr: &str, listenPort: i32) -> Result<(), Error> {

    let bind_addr = format!("{}:{}", listenAddr, listenPort);
    let mut socket = try!(UdpSocket::bind(&bind_addr.as_str()));
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
        try!(socket.send_to(buf, &src));

        // quit if instructed to do so
        // "&" in front of message converts String type to &str type
        if &message == "QUIT"{
            break;
        }
    }
    Ok(())
}   // the socket is closed here



pub fn socket_send() -> Result<(), Error> {

    let mut socket = try!(UdpSocket::bind("0.0.0.0:0"));
    socket.set_broadcast(true);
    println!("{:?}", socket);

    // put message here
    let array:&[u8] = b"QUIT";
    try!(socket.send_to(&array, "255.255.255.255:13389"));

    let mut buf = [0; 255];
    let (amt, src) = try!(socket.recv_from(&mut buf));
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
