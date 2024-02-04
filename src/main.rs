pub(crate) mod handlers;
pub(crate) mod models;

use std::{env, io};

use actix_files::Files;
use actix_web::{
    main,
    middleware::Logger,
    web::resource,
    web::{get, post, Data},
    App, HttpServer,
};
use handlers::{create_todo, todos};
use sqlx::postgres::PgPoolOptions;

#[main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"))
        .await
        .expect("failed to connect to database");
    HttpServer::new(move || {
        let logger = Logger::new("%a %r %s");
        App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .service(Files::new("/web", "./web"))
            .service(
                resource("todos")
                    .route(get().to(todos))
                    .route(post().to(create_todo)),
            )
    })
    .bind(env::var("LISTEN_ADDR").expect("LISTEN_ADDR environment variable not set"))
    .expect("Failed to bind address")
    .run()
    .await
}
