use employee_directory::{db, create_app};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use axum::http::{Method, header, HeaderValue};

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let pool = db::init_pool().await;
    
    // Define CORS policy
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]);
    
    // Wrap your app with the CORS layer
    let app = create_app(pool).layer(cors);
    
    // Note: Your code uses port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}