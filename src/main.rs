#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

use rocket::{self, get, routes, catch, catchers};
use rocket_contrib::json::{JsonValue};
use rocket_contrib::json;

mod schema;
mod service;
mod db;
mod model;
mod auth;


#[get("/")]
fn greeting(_auth: auth::Auth) -> &'static str{
    "Welcome!"
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(403)]
fn forbidden() -> JsonValue {
    json!({
        "status": "error",
        "reason": "403: Access Forbidden"
    })
}

fn main() {
    rocket::ignite()
        .attach(db::DbConn::fairing())
        .mount("/", routes![greeting])
        .mount("/product", routes![service::product::get_products, service::product::create_product])
        .mount("/orders", routes![service::orders::get_orders, service::orders::create_order])
        .register(catchers![not_found, forbidden])
        .launch();
}
