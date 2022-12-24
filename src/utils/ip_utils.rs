use std::net::{Ipv4Addr, TcpStream};

fn is_local_ipv4_address(ip_address: &Ipv4Addr) -> bool {
    match TcpStream::connect((ip_address, 80)) {
        Ok(stream) => stream.local_addr().unwrap().ip() == *ip_address,
        Err(_) => false,
    }
}
