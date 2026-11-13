use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder, get, post,
    web::{self},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection};
use serde::Deserialize;
use soul_api::{common, entity};

#[derive(Deserialize)]
struct PostDto {
    content: String,
    user_id: i64,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("success")
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    HttpResponse::Ok().body("todo")
}

#[post("/post")]
async fn create_post(
    data: web::Data<AppState>,
    post_info: web::Json<PostDto>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let post = entity::post::ActiveModel {
        content: Set(post_info.content.to_owned()),
        user_id: Set(post_info.user_id),
        ..Default::default()
    };

    let _post: entity::post::Model = post.insert(conn).await.unwrap();

    Ok(HttpResponse::Found().finish())
}

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    common::init_logger();
    tracing::info!("初始化日志成功");
    // 连接数据库
    let db = Database::connect("postgres://postgres:200011@127.0.0.1/soul")
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();
    let state = AppState { conn: db };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/health_check", web::get().to(health_check))
            .service(create_post)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
