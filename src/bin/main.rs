use std::fs::File;
use std::io::BufReader;

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
        App::new()
            // .on_connect(get_conn_info)
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(no_params)
    })
    .bind_rustls("127.0.0.1:8443", config)?
    .run()
    .await
}
