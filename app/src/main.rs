use axum::{
    routing::{get, post},
    extract::{Form, State},
    response::{Html, IntoResponse},
    Router,
};
use http::Method;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};

#[derive(serde::Deserialize)]
struct NewMessage {
    message: String,
}

type Messages = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let messages = Arc::new(Mutex::new(vec![]));
    let app = Router::new()
        .route("/api/messages", get(get_messages))
        .route("/api/send-message", post(send_message))
        .with_state(messages.clone())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_messages(State(messages): State<Messages>) -> impl IntoResponse {
    let messages = messages.lock().unwrap();
    let mut html = String::new();
    
    for msg in messages.iter() {
        html.push_str(&format!("<div class='message'><strong>User:</strong> {}</div>", msg));
    }
    Html(html)
}

async fn send_message(
    State(messages): State<Messages>,
    Form(input): Form<NewMessage>,
) -> impl IntoResponse {
    let mut messages = messages.lock().unwrap();
    messages.push(input.message.clone());
    Html(format!("<div class='message'><strong>You:</strong> {}</div>", input.message))
}