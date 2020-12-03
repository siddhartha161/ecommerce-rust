table! {
    line_item (id) {
        id -> Int4,
        orders -> Nullable<Int4>,
        product -> Int4,
        quantity -> Int4,
    }
}

table! {
    orders (id) {
        id -> Int4,
        email -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    product (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(line_item -> orders (orders));
joinable!(line_item -> product (product));

allow_tables_to_appear_in_same_query!(
    line_item,
    orders,
    product,
);
