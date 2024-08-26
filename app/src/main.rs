use std::rc::Rc;

use axum::{
    routing::{get, post},
    Router,
    error_handling::HandleErrorLayer
};
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;

use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use std::time::Duration;


use rusqlite::{Connection, Result};

mod views;
use views::*;

//type Messages = Arc<Mutex<Vec<String>>>;


#[tokio::main]
async fn main() {
    let conn = Connection::open("db/chat.db").expect("Database couldn't open");
    let shared_conn = Arc::new(Mutex::new(conn));

    let cors = CorsLayer::permissive();
    //let messages = Arc::new(Mutex::new(vec![]));
    let app = Router::new()
        .route("/api/messages", get(get_messages))
        .route("/api/send-message", post(send_message))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .with_state(Arc::clone(&shared_conn))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

