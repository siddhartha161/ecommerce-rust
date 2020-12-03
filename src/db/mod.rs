pub mod product;
pub mod orders;

use rocket_contrib::databases::{database, diesel::PgConnection};

#[database("postgres")]
pub struct DbConn(PgConnection);