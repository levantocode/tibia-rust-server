use log::error;
use std::net::{TcpStream, SocketAddr, Shutdown};
use std::io::{Read, Write};

use super::socket_utils;

pub fn process_data(mut stream: TcpStream) {
    let mut data_buffer: [u8; 50] = get_50_byte_buffer();

    while match stream.read(&mut data_buffer) {
        Ok(incoming_data_bytes_size) => {
            handle_stream_data_processing(&stream, &data_buffer, incoming_data_bytes_size);
            true
        },

        Err(_) => {
            handle_stream_reading_error(&stream);
            false
        }
    } {}
}

fn get_50_byte_buffer() -> [u8; 50] {
    [0 as u8; 50]
}

fn handle_stream_data_processing(stream: &TcpStream, data_buffer: &[u8; 50], incoming_data_bytes_size: usize) {
    let data_bytes: &[u8] = &data_buffer[0..incoming_data_bytes_size];
    prints_everything_on_console(&stream, data_bytes);
}

fn prints_everything_on_console(mut stream: &TcpStream, data_bytes: &[u8]) {
    stream.write(data_bytes).unwrap();
}

fn handle_stream_reading_error(stream: &TcpStream) {
    let troubling_socket_address: SocketAddr = socket_utils::get_socket_address_from(&stream);
    error!("An error occurred, terminating connection with {}", troubling_socket_address);

    terminate_connection_with(&stream);
}

fn terminate_connection_with(stream: &TcpStream) {
    stream.shutdown(Shutdown::Both).unwrap();
}
