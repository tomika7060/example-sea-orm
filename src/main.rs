use log::info;
use sea_orm_example::infrastructure::server;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting server...");
    server::run()
}
