table! {
    brands (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    product_categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    products (id) {
        id -> Integer,
        code -> Text,
        name -> Text,
        category_id -> Integer,
        brand_id -> Integer,
        photo_path -> Nullable<Text>,
    }
}

joinable!(products -> brands (brand_id));
joinable!(products -> product_categories (category_id));

allow_tables_to_appear_in_same_query!(
    brands,
    product_categories,
    products,
);
