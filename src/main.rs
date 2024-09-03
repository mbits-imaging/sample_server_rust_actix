use actix_web::{web, App, HttpServer};
use diesel::{r2d2, SqliteConnection};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

mod db;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Pretty env logger
    pretty_env_logger::init();

    // Connect to SQLite DB
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new("app.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
