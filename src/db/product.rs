use rocket_contrib::json::Json;
use diesel::{result::Error, prelude::*};
use crate::schema::product;
use crate::model::product::{Product, NewProduct};

pub fn get_products(connection: &PgConnection) -> Vec<Product> {
    product::table
            .order(product::columns::id.desc())
            .load::<Product>(&*connection)
            .expect("error in fetch")
}


pub fn create_product(connection: &PgConnection, new_product: Json<NewProduct>) -> Product {
    diesel::insert_into(product::table)
            .values(&*new_product)
            .get_result(&*connection)
            .expect("error in saving")
}

pub fn get_product_by_id(connection: &PgConnection, id: i32) -> Product {
    get_product_result_by_id(connection, id).expect("error product not found")
}

pub fn get_product_result_by_id(connection: &PgConnection, id: i32) ->  Result<Product, Error> {
    product::table
            .filter(product::id.eq(id))
            .get_result::<Product>(&*connection)
}

pub fn validate_products(connection: &PgConnection, ids: Vec<i32>) -> bool {
    for id in ids {
        let product = get_product_result_by_id(connection, id);
        match product {
            Ok(_) => continue,
            Err(err) => match err {
                diesel::result::Error::NotFound => return false,
                _ => panic!("unkown error {} ", err)
            }
        }
    }
    true
}