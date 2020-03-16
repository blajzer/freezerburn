use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::models::*;

pub struct ProductCategoryService {
    conn: SqliteConnection,
}

impl ProductCategoryService {
    fn create(&self, product_category: NewProductCategory) {
        use super::schema::product_categories;

        diesel::insert_into(product_categories::table)
            .values(&product_category)
            .execute(&self.conn)
            .expect("Error creating product category");
    }

    fn list(&self, limit: Option<i64>, offset: Option<i64>) -> Vec<ProductCategory> {
        use super::schema::product_categories::dsl::*;

        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        product_categories.limit(limit)
            .offset(offset)
            .load::<ProductCategory>(&self.conn)
            .expect("Error listing product categories")
    }
}

#[test]
fn my_test() {
    assert!(true);
}