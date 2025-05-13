mod routes;
mod models;
use actix_web::{ App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(routes::health)
            .service(routes::add_entry)
            .service(routes::get_entries)
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
