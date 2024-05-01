use actix_web::{web, App, HttpServer};
use rustback::router;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_string = String::from("postgres://postgres:password@localhost:5432/postgres");
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(router::create_user)
            .service(router::get_user)
            .service(router::get_users)
        // Add routes for update and delete
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
