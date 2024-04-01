mod api;
mod config;
mod db;
pub mod entities;
mod error;
mod repository;
mod service;

use std::sync::Arc;

use config::Config;
pub use error::Error;
pub use repository::Repository;
pub use service::Service;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let config = Config::load()?;

    let db_pool = db::connect(&config.database_url).await?;
    db::migrate(&db_pool).await?;

    let service = Service::new(db_pool);
    let app_state = Arc::new(api::AppState::new(service));

    let routes = api::routes::routes(app_state);

    Ok(())

}
