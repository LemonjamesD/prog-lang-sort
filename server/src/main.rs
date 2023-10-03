pub mod list_of_langs;

use crate::list_of_langs::SERIALIZED_LANGS;

use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/langs")]
async fn langs(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(SERIALIZED_LANGS.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo) // From getting started
            .service(langs)
            .service(Files::new("/", "../client/dist") // File server needed for Svelte GUI, has to be created as last
                .prefer_utf8(true)
                .index_file("index.html")
            )
    })
    .bind(("127.0.0.1", 8080))? // Sample bind
    .run()
    .await
}