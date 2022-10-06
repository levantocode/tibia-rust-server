use log::info;

mod tcp;

fn main() {
    env_logger::init();

    info!("Starting Server . . .");
    tcp::open_server_connection();
}


mod main {

    mod tests {

        #[test]
        fn test() {
            assert!(true)
        }
    }
}
