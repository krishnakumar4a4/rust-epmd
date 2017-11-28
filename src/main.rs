use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:4369")
    						.expect("Couldn't connect to the server...");
    connect(&mut stream);
    // ask_port(&mut stream);
    // let mut stream2 = TcpStream::connect("127.0.0.1:4369")
    						// .expect("Couldn't connect to the server...");
    ask_names(&mut stream);
}


fn connect(stream: &mut TcpStream) {
    let s_port = format!("{:x}", stream.local_addr().unwrap().port());
    println!("port {:?}", stream.local_addr().unwrap().port());

    let mut bufVec: [u8; 22] = [0; 22];
    bufVec[0] = 0;
    bufVec[1] = 20;
    bufVec[2] = "x".as_bytes()[0];
    bufVec[3] = s_port.clone().into_bytes()[0];
    bufVec[4] = s_port.into_bytes()[1];
    bufVec[5] = 77;
    bufVec[6] = 0;
    bufVec[7] = 0;
    bufVec[8] = 5;
    bufVec[9] = 0;
    bufVec[10] = 5;
    bufVec[11] = 0;
    bufVec[12] = 7;
    bufVec[13] = "k".as_bytes()[0];
    bufVec[14] = "r".as_bytes()[0];
    bufVec[15] = "i".as_bytes()[0];
    bufVec[16] = "s".as_bytes()[0];
    bufVec[17] = "h".as_bytes()[0];
    bufVec[18] = "n".as_bytes()[0];
    bufVec[19] = "a".as_bytes()[0];
    bufVec[20] = 0;
    bufVec[21] = 0;
    stream.write_all(&bufVec).unwrap();
    println!("wrote ");
    // let mut r_buf = String::new();
    let mut r_array: [u8; 4] = [0; 4];
    let r_len = stream.read(&mut r_array).unwrap();
    println!("read {:?} of length {}", r_array, r_len);
}

fn ask_port(stream: &mut TcpStream) {
	let mut buf_vec: [u8; 10] = [0; 10];
	buf_vec[0] = 0;
	buf_vec[1] = 10;
	buf_vec[2] = "z".as_bytes()[0];
	buf_vec[3] = "k".as_bytes()[0];
    buf_vec[4] = "r".as_bytes()[0];
    buf_vec[5] = "i".as_bytes()[0];
    buf_vec[6] = "s".as_bytes()[0];
    buf_vec[7] = "h".as_bytes()[0];
    buf_vec[8] = "n".as_bytes()[0];
    buf_vec[9] = "a".as_bytes()[0];
    stream.write_all(&buf_vec).unwrap();
    println!("asked");
    let mut r_array: [u8; 4] = [0; 4];
    let r_len = stream.read(&mut r_array).unwrap();
    println!("ask - read {:?} of length {}", r_array, r_len);
}


fn ask_names(stream: &mut TcpStream) {
	let mut buf_vec: [u8; 3] = [0; 3];
	buf_vec[0] = 0;
	buf_vec[1] = 1;
	buf_vec[2] = "n".as_bytes()[0];
    stream.write_all(&buf_vec).unwrap();
    println!("asked names");
    let mut r_array: [u8; 30] = [0; 30];
    let mut r_str = String::new();
    // let r_len = stream.read(&mut r_array).unwrap();
    let r_len = stream.read_to_string(&mut r_str).unwrap();
    println!("ask - read names {:?} of length {}", r_str, r_len);
}