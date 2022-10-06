use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddr};

use super::stream_data_processor;
use super::socket_utils;


pub fn open_server_connection() {
    
    let listener: TcpListener = socket_utils::start_listening_to_socket();
    println!("Server listening on: {}", socket_utils::get_server_socket_address());
    
    for incoming_data_stream in listener.incoming() {
        match incoming_data_stream {
            Ok(stream) => spawn_thread_to_process_data(stream),
            Err(err) => println!("Error: {}", err)
        }
    }

    drop(listener); // close the socket server
}

fn spawn_thread_to_process_data(stream: TcpStream) {
    recognize_new_client_connection(&stream);
    
    thread::spawn(move ||
        stream_data_processor::process_data(stream)
    );
}

fn recognize_new_client_connection(stream: &TcpStream) {
    let connected_socket_address: SocketAddr = socket_utils::get_socket_address_from(&stream);
    println!("New connection: {}", connected_socket_address);
}
