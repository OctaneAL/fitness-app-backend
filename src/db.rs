use tokio_postgres::NoTls;

use std::env;
use tokio::sync::Mutex;
use std::sync::Arc;

pub type DbPool = Arc<tokio::sync::Mutex<tokio_postgres::Client>>;

pub async fn connect() -> Result<DbPool, tokio_postgres::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(Mutex::new(client));

    Ok(client)
}