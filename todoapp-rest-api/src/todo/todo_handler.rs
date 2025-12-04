use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use diesel::{QueryDsl, SelectableHelper};
use serde::Serialize;
use diesel::prelude::*;

use crate::DbPool;
use crate::todo::model::Todo;
use crate::schema::todos;

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub completed: Option<bool>,
}


impl From<Todo> for TodoResponse {
    fn from(post: Todo) -> Self {
        TodoResponse {
            id: post.id,
            title: post.title,
            completed: post.completed,
        }
    }
}

pub async  fn get_todo(State(pool): State<DbPool>) -> Result<Json<Vec<TodoResponse>>, (StatusCode, String)>{

    let mut connection = pool.connection.lock().unwrap();

    let results = todos::table
        .select(Todo::as_select())
        .load(&mut *connection)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // one line
    // let response: Vec<TodoResponse> = results.into_iter().map(|p| p.into()).collect();
    let mut response = Vec::new();
        for p in results {
            response.push(TodoResponse::from(p));
        }

    Ok(Json(response))

}