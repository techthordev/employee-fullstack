use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

/// Initializes a connection pool to the PostgreSQL database.
/// A 'Pool' allows multiple database connections to be reused, which is
/// much faster than opening a new connection for every single request.
pub async fn init_pool() -> PgPool {
    // Look for the "DATABASE_URL" variable in your system environment or .env file.
    // If it's missing, the program will crash with the message provided in .expect().
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Configure how the connection pool should behave.
    PgPoolOptions::new()
        // Allow up to 5 simultaneous connections to the database.
        .max_connections(5)
        // Attempt to establish the connection using the URL we found above.
        // This is 'async', so we must .await the result.
        .connect(&database_url)
        .await
        // If the connection fails (e.g., wrong password or DB is down),
        // stop the program and show an error.
        .expect("Can't connect to database")
}
