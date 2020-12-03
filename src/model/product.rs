
use crate::schema::product;
use chrono::{ DateTime, Utc };
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};


#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>
}


#[derive(Insertable, Deserialize)]
#[table_name="product"]
pub struct NewProduct {
    name: String
}
