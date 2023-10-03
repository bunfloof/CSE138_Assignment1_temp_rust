use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "world"}))
}

#[post("/hello")]
async fn hello_post() -> impl Responder {
    HttpResponse::MethodNotAllowed().body("Method Not Allowed")
}

#[post("/hello/{name}")]
async fn hello_name(info: web::Path<NameInfo>) -> impl Responder {
    HttpResponse::Ok().json(json!({"message": format!("Hi, {}.", info.name)}))
}

#[get("/hello/{name}")]
async fn hello_name_get() -> impl Responder {
    HttpResponse::MethodNotAllowed().body("Method Not Allowed")
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "test is successful"}))
}

#[post("/test")]
async fn test_post(web::Query(info): web::Query<MsgQuery>) -> impl Responder {
    match info.msg {
        Some(msg) => HttpResponse::Ok().json(json!({"message": msg})),
        None => HttpResponse::BadRequest().body("Bad Request"),
    }
}

#[derive(Deserialize)]
struct MsgQuery {
    msg: Option<String>,
}

#[derive(Deserialize)]
struct NameInfo {
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_post)
            .service(hello_name)
            .service(hello_name_get)
            .service(test)
            .service(test_post)
    })
    .bind("0.0.0.0:8090")?
    .run()
    .await
}
