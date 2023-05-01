use actix_web::{HttpServer, App};

mod routes;
use routes::*;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server =HttpServer::new(|| App::new().service(home).service(hello_user).service(create_new_user))
    .bind(("127.0.0.1", 8000))?
    .run();

    println!("Server Running at 127.0.0.1");
    server.await
}
