use actix_web::{web, App, HttpServer};
mod free_slot;

async fn api_index(input: String) -> String {
    match free_slot::free_slots(&input) {
        Ok(resp) => resp,
        Err(err) => format!("{}", err),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api", web::post().to(api_index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
