use log::{info, error};
use dotenv;

mod connections;

use connections::tcp::tcp_server;
use connections::data_base::database_connector;


#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    info!("Starting Database . . .");

    match database_connector::init_db().await {
        Ok(_) => info!("Database Successfully Started"),
        Err(err) => error!("Failed to start Database. Reason: {:?}", err)
    };

    start_ftp_server()
}

fn start_ftp_server() {
    info!("Starting Server . . .");
    tcp_server::open_server_connection();
}

mod tests {

    #[test]
    fn test() {
        assert!(true)
    }
}