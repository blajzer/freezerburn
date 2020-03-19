use actix_web::{App, HttpServer};

use freezerburn::apis;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(apis::list_product_categories)
            .service(apis::create_product_category)
            .service(apis::delete_product_category)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}