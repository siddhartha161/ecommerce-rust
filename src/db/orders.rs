
use crate::db::product::{get_product_by_id, validate_products};
use crate::model::product::Product;
use rocket_contrib::json::Json;
use diesel::prelude::*;
use crate::{schema::orders};
use crate::schema::line_item;
use crate::model::orders::*;

pub fn get_orders(connection: &PgConnection) -> Vec<OrdersJson> {
    orders::table
                .order(orders::columns::id.desc())
                .load::<Orders>(&*connection)
                .expect("error in fetching orders")
                .into_iter()
                .map(|order| -> OrdersJson {
                 let id = order.id;
                 order.attach(get_line_items(&connection, id))})
                .collect()
}

pub fn create_order(connection: &PgConnection, new_order_wrapper: Json<NewOrderWrapper>) -> Option<OrdersJson> {
    let new_order: &NewOrder = &new_order_wrapper.order;
    // validate products
    if !validate_products(&connection, new_order.line_items.iter().map(|item| item.product_id).collect()) { return None;}

    let email =  &new_order.email;
    let result: Orders = diesel::insert_into(orders::table)
        .values(&*get_order_json(String::from(email)))
        .get_result(&*connection)
        .expect("error in saving");

    let items: Vec<LineItemRespone> = new_order.line_items.iter().map(|line_item| -> LineItemRespone {
        create_line_item(&connection, &line_item, result.id)
    }).collect();

    Some(OrdersJson{id: result.id, email: result.email, line_items: items})
}

fn get_order_json(email: String) -> Json<Order> {
    Json(Order { email })
}

fn create_line_item(connection: &PgConnection, line_item: &NewLineItem, orderid: i32) -> LineItemRespone {
    let id = line_item.product_id;
    let prouct = get_product_by_id(connection, id);
    let line_item_record = LineItemTable {
        product: prouct.id,
        orders: Some(orderid),
        quantity: line_item.quantity
    };
    diesel::insert_into(line_item::table)
        .values(line_item_record)
        .get_result::<LineItem>(connection)
        .expect("error in inserting line item")
        .attach(prouct)
}



fn get_line_items(connection: &PgConnection, id: i32) -> Vec<LineItemRespone> {
    line_item::table
                .filter(line_item::orders.eq(id))
                .load::<LineItem>(connection)
                .expect("product not found")
                .into_iter()
                .map(|line| -> LineItemRespone {
                    let product: Product = get_product_by_id(connection, line.product);
                    line.attach(product)
                })
                .collect()
                

}