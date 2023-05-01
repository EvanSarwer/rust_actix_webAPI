use actix_web::{get, Responder};

#[get("/home")]
async fn home() -> impl Responder {
    let response = "Welcome to actix web server";
    response
}
