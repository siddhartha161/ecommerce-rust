
use rocket::{self, get, post};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use crate::model::orders::*;
use crate::db::DbConn;
use crate::db;
use crate::auth::Auth;


#[get("/")]
pub fn get_orders(connection: DbConn, _auth: Auth) -> JsonValue {
    let orders = db::orders::get_orders(&connection);
    json!( {"orders": orders})
}


#[post("/", data = "<new_order_wrapper>")]
pub fn create_order(connection: DbConn, _auth: Auth, new_order_wrapper: Json<NewOrderWrapper>) -> JsonValue {
    let orders = db::orders::create_order(&connection, new_order_wrapper);
    match orders {
        Some(orders) => json!({"orders": orders}),
        None => json!({"error": "error in creating orders"})
    }
}