
use crate::{schema::orders};
use crate::schema::line_item;
use chrono::{ DateTime, Utc };
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::model::product::Product;

#[derive(Queryable, Serialize)]
pub struct  Orders {
    pub id: i32,
    pub email: String,
    pub created_at: DateTime<Utc>
}

impl Orders {
    pub fn attach(self, line_items: Vec<LineItemRespone>) -> OrdersJson {
        OrdersJson {
            id: self.id,
            email: self.email,
            line_items
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name="orders"]
pub struct Order {
    pub email: String
}

#[derive(Serialize)]
pub struct OrderWrapper {
    pub orders: Vec<OrdersJson>
}

#[derive(Deserialize)]
pub struct NewOrder {
    pub email: String,
    pub line_items: Vec<NewLineItem>
}

#[derive(Deserialize)]
pub struct NewOrderWrapper {
    pub order: NewOrder
}

#[derive(Deserialize)]
pub struct NewLineItem {
    pub quantity: i32,
    pub product_id: i32
}

#[derive(Queryable, Serialize)]
pub struct  LineItem {
    #[serde(skip_serializing)]
    pub id: i32,
    pub orders: Option<i32>,
    pub product: i32,
    pub quantity: i32
}

impl LineItem {
    pub fn attach(self, product: Product) -> LineItemRespone {
        LineItemRespone {
            id: self.id,
            product,
            quantity: self.quantity
        }
    }
}


#[derive(Queryable, Serialize)]
pub struct  LineItemRespone {
    #[serde(skip_serializing)]
    pub id: i32,
    pub product: Product,
    pub quantity: i32
}

#[derive(Serialize)]
pub struct OrdersJson {
    pub id: i32,
    pub email: String,
    pub line_items: Vec<LineItemRespone>
}

#[derive(Insertable, Deserialize)]
#[table_name="line_item"]
pub struct LineItemTable {
    pub orders: Option<i32>,
    pub product: i32,
    pub quantity: i32
}