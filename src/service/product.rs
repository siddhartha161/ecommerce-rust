use rocket::{self, get, post};
use rocket_contrib::json::{ Json, JsonValue };
use rocket_contrib::json;
use crate::db::{ self,DbConn};
use crate::model::product::NewProduct;
use crate::auth::Auth;


#[get("/")]
pub fn get_products(connection: DbConn, _auth: Auth) -> JsonValue {
    let products = db::product::get_products(&connection);
    json!({ "products": products })
}

#[post("/", format = "json", data = "<new_product>")]
pub fn create_product(connection: DbConn, _auth: Auth, new_product: Json<NewProduct>) -> Result<JsonValue, &'static str> {
    let result = db::product::create_product(&connection, new_product);
    Ok(json!({"product": result}))
}