use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddr};

mod stream_data_processor;


static TCP_SERVER_ADDRESS: &str = "127.0.0.1:3333";


pub fn open_server_connection() {
    let listener: TcpListener = start_listening_to_socket();
    println!("Server listening on: {}", TCP_SERVER_ADDRESS);
    
    for incoming_data_stream in listener.incoming() {
        match incoming_data_stream {
            Ok(stream) => spawn_thread_to_process_data(stream),
            Err(err) => println!("Error: {}", err)
        }
    }

    drop(listener); // close the socket server
}

pub fn start_listening_to_socket() -> TcpListener{
    TcpListener::bind(TCP_SERVER_ADDRESS).unwrap()
}

fn spawn_thread_to_process_data(stream: TcpStream) {
    recognize_new_client_connection(&stream);
    
    thread::spawn(move ||
        stream_data_processor::process_data(stream)
    );
}

fn recognize_new_client_connection(stream: &TcpStream) {
    let connected_socket_address: SocketAddr = stream_data_processor::get_socket_address_from(&stream);
    println!("New connection: {}", connected_socket_address);
}

