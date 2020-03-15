use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::models::*;

pub struct ProductCategoryService {
    conn: SqliteConnection,
}

impl ProductCategoryService {
    fn create(&self, product_category: NewProductCategory) -> ProductCategory {
        diesel::insert_into(product_categories::table)
            .values(&product_category)
            .get_result(&self.conn)
            .expect("Error saving new product category")
    }

    fn list(&self, limit: Option<int32>, offset: Option<int32>) -> Vec<ProductCategory> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        product_categories
            .limit(limit)
            .offset(offset)
            .load::<ProductCategory>(&self.conn)
    }
}