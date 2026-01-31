// Import the modules and the create_app function from our own library.
// 'employee_directory' is the name defined in your Cargo.toml file.
use employee_directory::{db, create_app};
use std::net::SocketAddr;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // 1. Load variables from the .env file into the environment.
    dotenv().ok();
    
    // 2. Initialize the database connection pool (logic is in db.rs).
    let pool = db::init_pool().await;
    
    // 3. Create the app router (logic is in lib.rs).
    let app = create_app(pool);
    
    // 4. Define the network address (localhost on port 3000).
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started at http://{}", addr);
    println!("📖 Swagger UI: http://{}/swagger-ui", addr);
    
    // 5. Create a listener and start serving the app.
    // This will run forever until you stop the program (Ctrl+C).
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}