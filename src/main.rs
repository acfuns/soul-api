use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use soul_api::common;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    common::init_logger();
    tracing::info!("初始化日志成功");

    // 连接数据库
    let db = Database::connect("postgres://posgres:200011@localhost/soul")
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
