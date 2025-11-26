use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor Actix Web ejecutándose en http://127.0.0.1:8090/ ...");

    HttpServer::new(|| {
        App::new().service(
            Files::new("/", "./src/static")
                .index_file("index.html")
                .show_files_listing(),
        )
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}