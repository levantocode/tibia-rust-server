use log::info;
use dotenv;

mod connections;

use connections::tcp::tcp_server;


fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    info!("Starting Server . . .");
    tcp_server::open_server_connection();
}

mod tests {

    #[test]
    fn test() {
        assert!(true)
    }
}