use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self},
};
use soul_api::common;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    common::init_logger();
    tracing::info!("初始化日志成功");
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
