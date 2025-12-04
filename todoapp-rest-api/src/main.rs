use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::get;

mod schema;
mod todo;

use todo::todo_handler;
#[derive(Clone)]
pub struct DbPool {
    pub connection : Arc<Mutex<MysqlConnection>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let db_pool = DbPool {
        connection: Arc::new(Mutex::new(connection)),
    };

    let todo_router = Router::new()
        .route("/todo", get(todo_handler::get_todo));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", todo_router)
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Started http://localhost:3000/");

    axum::serve(listener, app).await.unwrap();
}
