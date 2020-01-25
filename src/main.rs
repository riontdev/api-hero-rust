//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use dotenv;
use listenfd::ListenFd;
use std::env;

mod models;
mod schema;
mod handlers;
mod db;
mod api_error;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

    //let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    //let manager = ConnectionManager::<PgConnection>::new(connspec);
    // let pool = db::connection();
    //r2d2::Pool::builder()
      //  .build(manager)
        //.expect("Failed to create pool.");
    db::init();
    // Start http server
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move ||
        App::new()
        //.data(pool.clone())
        // enable logger
        .wrap(middleware::Logger::default())
        .configure(handlers::hero::init_routes)
    );
  
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
