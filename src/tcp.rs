use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddr};

mod stream_data_processor;


static TCP_SERVER_ADDRESS: &str = "127.0.0.1:3333";


pub fn open_server_connection() {
    let listener: TcpListener = get_socket_listener();
    println!("Server listening on: {}", TCP_SERVER_ADDRESS);
    
    for incoming_data_stream in listener.incoming() {
        match incoming_data_stream {
            Ok(stream) => spawn_thread_to_process_data(stream),
            Err(err) => println!("Error: {}", err)
        }
    }

    drop(listener); // close the socket server
}

pub fn get_socket_listener() -> TcpListener{
    TcpListener::bind(TCP_SERVER_ADDRESS).unwrap()
}

fn spawn_thread_to_process_data(stream: TcpStream) {
    let connected_socket_address: SocketAddr = stream_data_processor::get_socket_address_from(&stream);
    println!("New connection: {}", connected_socket_address);

    thread::spawn(move || stream_data_processor::process_data(stream));
}

