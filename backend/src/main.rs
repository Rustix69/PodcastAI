mod api;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    is_main();
    HttpServer::new(|| {
        App::new().configure(api::routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn is_main() {
    println!("\n 🖥️  Server running on port 8080 \n");
}