
use axum::{
    Router, routing::{get, post}
};
use std::sync::{Arc, Mutex};

mod vehicle;

#[derive(Clone)]
struct AppState {
    persons: Arc<Mutex<Vec<vehicle::Person>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        persons: Arc::new(Mutex::new(vec![
            vehicle::Person {
                first_name: "Ardiman".to_string(),
                last_name: "xyz".to_string(),
                email: "ardiman@umkendari.ac.id".to_string(),
                age: 29,
            }
        ])),
    };

    let api_routes = Router::new()
             .route("/vehicle", get(vehicle::vehicle_get))
             .route("/vehicle", post(vehicle::vehicle_post));

    let app = Router::new()
                .route("/", get(|| async { "Hello, world!" }))
                .nest("/api", api_routes)
                .with_state(state);
    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Started http://localhost:3000/");
    
    axum::serve(listener, app).await.unwrap();

}