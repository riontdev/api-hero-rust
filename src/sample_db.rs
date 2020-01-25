#[macro_use]
//extern crate diesel;
//extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use std::ops::Deref;


pub type Manager = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

// For the convenience of using an &Connection as an &PgConnection.
impl Deref for Connection {
	type Target = PgConnection;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}


pub fn get_pool() -> Manager {
	
	let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
		.expect("Failed to create pool.");
	return pool;
  }

