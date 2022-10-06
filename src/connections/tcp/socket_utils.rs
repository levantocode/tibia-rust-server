use std::net::{TcpStream, TcpListener, SocketAddr};

use dotenv;


pub fn start_listening_to_socket() -> TcpListener{
    let tcp_server_address: String = get_server_socket_address();
    TcpListener::bind(tcp_server_address).unwrap()
}

pub fn get_server_socket_address() -> String {
    dotenv::var("TCP_SERVER_ADDRESS").unwrap()
}

pub fn get_socket_address_from(stream: &TcpStream) -> SocketAddr {
    stream.peer_addr().unwrap()
}
