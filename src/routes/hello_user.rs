use actix_web::{web::{Json, Path}, get, Responder, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

impl User {
     fn new(firstname: String, lastname: String)-> Self {
        Self { first_name: firstname, last_name: lastname }
     }
}

#[get("/hello/{firstname}/{lastname}")]
pub async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    // let response: String = format!("Hello {} {}", params.0, params.1);
    let response =User::new(params.0.clone(), params.1.clone());
    (Json(response), StatusCode::OK)
}

