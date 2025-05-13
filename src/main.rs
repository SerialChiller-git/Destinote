mod config;
mod models;
mod initDB;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use models::Diary;
use sqlx::PgPool;
use actix_cors::Cors;

#[get("/hello/{name}")]
async fn greet(name : web::Path<String>) -> impl Responder{
    HttpResponse::Ok()
        .json(models::Status {
            status : format!("Hello {}!", name).to_string(),
        })
}

#[get("/api/diary/{id}")]
async fn get_diary(pool : web::Data<PgPool>, id : web::Path<i64>) -> impl Responder{
    let entry = initDB::get_entry(&pool, id.into_inner()).await;
    HttpResponse::Ok()
        .json(entry)
}

#[put("/api/diary/{id}")]
async fn update_diary(pool: web::Data<PgPool>, id: web::Path<i64>, text: web::Json<String>) -> impl Responder {
    let id = id.into_inner();
    initDB::update_entry(&pool, id, &text).await;
    HttpResponse::Ok()
        .json(id)
    
}

#[post("/api/create")]
async fn create(pool : web::Data<PgPool>, new_diary : web::Json<Diary>) -> impl Responder{
    let id : i64 =initDB::new_entry(&pool, &new_diary).await;
    initDB::new_text_entry(&pool, id).await;
    HttpResponse::Ok()
        .json(id)
}

#[tokio::main]
async fn main() -> std::io::Result<()>{

    dotenv::dotenv().ok();
    let conf = crate::config::Config::from_env(); 
    let database_url = &conf.db;
    let pool = PgPool::connect(&database_url).await.unwrap();

    //Initialize tables of Diary
    initDB::defaults(&pool).await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();    

        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(cors)
            .service(greet)
            .service(create)

    })
    .bind(format!("{}:{}", conf.server.host, conf.server.port))?
    .run()
    .await
}