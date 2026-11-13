use actix_web::{
<<<<<<< HEAD
    App, HttpResponse, HttpServer, Responder, get,
=======
    App, Error, HttpResponse, HttpServer, Responder, get, post,
>>>>>>> cc6105f
    web::{self},
};
use soul_api::common;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("success")
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    HttpResponse::Ok().body("todo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    common::init_logger();
    tracing::info!("初始化日志成功");
    tracing::info!("hello world");
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
