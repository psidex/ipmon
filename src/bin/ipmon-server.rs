use actix_web::middleware::Logger;
use actix_web::{dev::ConnectionInfo, get, App, HttpResponse, HttpServer, Responder};

use ipmon::platform;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn handler(conn: ConnectionInfo) -> impl Responder {
    match conn.realip_remote_addr() {
        Some(ip) => HttpResponse::Ok().body(ip.to_owned()),
        None => HttpResponse::BadRequest().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let ip = match platform::is_debug() {
        true => "127.0.0.1",
        false => "0.0.0.0",
    };

    // Log format from here, with real ip instad of peer ip:
    // https://docs.rs/actix-web/latest/actix_web/middleware/struct.Logger.html

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(health)
            .service(handler)
    })
    .bind((ip, 8080))?
    .run()
    .await
}
