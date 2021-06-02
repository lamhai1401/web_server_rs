// extern crate web_server_rs;
// use web_server_rs::utils::ssl::load_ssl;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::http::header;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer, Responder, Result};

#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

use web_server_rs::models::*;
use web_server_rs::schema::cats::dsl::*;

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

// async fn index() -> Result<NamedFile> {
//     Ok(NamedFile::open("./static/index.html")?)
// }

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Can't get db connection from pool");

    let cats_data: Vec<Cat> = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(cats_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let config = load_ssl();
    // Setting up the database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            // .on_connect(get_conn_info)
            .wrap(cors)
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .data(pool.clone())
            .service(no_params)
            .service(Files::new("/static", "static").show_files_listing())
            .route("/hello", web::get().to(hello))
            .route("/index", web::get().to(index))
    })
    .bind("127.0.0.1:8443")?
    // .bind_rustls("127.0.0.1:8443", config)?
    .run()
    .await
}
