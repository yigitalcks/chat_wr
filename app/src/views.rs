use std::{sync::{Arc, Mutex}, time::SystemTime};
use axum::{
    body::{Body, Bytes}, 
    extract::{Extension, Json, Path, Query, Request, State, Form}, 
    http::{header::HeaderMap, StatusCode}, response::Html
};
use tower_http::BoxError;

use rusqlite::{Connection, Result};
use std::time::{UNIX_EPOCH, Duration};

type Conn = Arc<Mutex<Connection>>;

struct Message {
    sender: String,
    reciever: String,
    send_dateandtime: SystemTime,
    message: String,
};

pub async fn get_messages(State(conn): State<Conn>) -> Result<Html<String>, StatusCode> {
    let conn = conn.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut stmt = conn.prepare("SELECT * from Message;")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let msgs = stmt.query_map([], |row| {
        let datetime_r: i64 = row.get(2)?;
        let date_time = UNIX_EPOCH + Duration::from_secs(datetime_r as u64);
        Ok(Message{
            sender: row.get(0)?,
            reciever: row.get(1)?,
            send_dateandtime: date_time,
            message: row.get(3)?,
        })
    }).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut history = String::new();
    for msg_result in msgs {
        let msg = msg_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        history.push_str(&format!("<div class='message'><strong>{}:</strong> {}</div>", msg.sender, msg.message));
    }

    Ok(Html(history))
}

pub async fn send_message(
    State(conn): State<Conn>,
    Form(input): Form<String>
) -> Result<StatusCode, StatusCode> {
    let mut conn = conn.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    conn.execute(
        "INSERT INTO Message (sender, receiver, send_dateandtime, message) VALUES (?, ?, strftime('%s'), ?)",
        ["You", "Server", &input]
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::ACCEPTED)
}

pub async fn create_table(State(conn): State<Conn>) {
    let conn = conn.lock().unwrap();
    conn.execute(
        "CREATE TABLE Message (
            sender TEXT NOT NULL,
            receiver TEXT NOT NULL,
            send_date INTEGER NOT NULL,  -- Store datetime as a Unix timestamp
            message TEXT NOT NULL
        )",
        (), // empty list of parameters.
    );
}

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}