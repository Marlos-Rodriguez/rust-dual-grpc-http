use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
pub async fn hello_http(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
