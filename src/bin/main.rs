use std::fs::File;
use std::io::BufReader;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

fn load_ssl() -> ServerConfig {
    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys: Vec<rustls::PrivateKey> = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_ssl();

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
    })
    // .bind("127.0.0.1:8443")?
    .bind_rustls("127.0.0.1:8443", config)?
    .run()
    .await
}
