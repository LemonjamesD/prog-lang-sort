use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo) // From getting started
            .service(Files::new("/", "../client/dist") // File server needed for Svelte GUI, has to be created as last
                .prefer_utf8(true)
                .index_file("index.html")
            )
    })
    .bind(("127.0.0.1", 8080))? // Sample bind
    .run()
    .await
}