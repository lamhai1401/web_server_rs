// extern crate web_server_rs;
// use web_server_rs::utils::ssl::load_ssl;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::http::header;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder, Result};

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let config = load_ssl();

    HttpServer::new(|| {
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

// sudo apt install libcanberra-gtk-module libcanberra-gtk3-module
