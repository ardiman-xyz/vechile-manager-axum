
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub first_name: String,
    pub last_name : String,
    pub email: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
pub struct JsonResponseApi {
    success: bool,
    message: String,
    data: Vec<Person>
}

pub async fn vehicle_get(State(state): State<AppState>) -> Json<JsonResponseApi> {
    let persons = state.persons.lock().unwrap();
    let data = persons.clone();

    let resonponse_api = JsonResponseApi {
        success: true,
        message: "Successfully get data".to_string(),
        data: data,
    };

    Json::from(resonponse_api)
}

pub async fn vehicle_post(State(state): State<AppState>) -> Json<JsonResponseApi> {
    let mut persons = state.persons.lock().unwrap();

    let new_person = Person {
        first_name: "Budi".to_string(),
        last_name: "Santoso".to_string(),
        email: "budi@example.com".to_string(),
        age: 25,
    };

    persons.push(new_person);

    let all_data = persons.clone();

    let response_api = JsonResponseApi {
        success: true,
        message: "Successfully added data".to_string(),
        data: all_data,
    };

    Json::from(response_api)
}