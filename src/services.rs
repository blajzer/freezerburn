use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::models::*;
use super::schema::product_categories;

#[derive(Insertable)]
#[table_name="product_categories"]
pub struct NewProductCategory {
    pub name: String,
}

pub struct ProductCategoryService {
    pub conn: SqliteConnection,
}

impl ProductCategoryService {
    pub fn create(&self, name: &String) {
        let product_category = NewProductCategory{ name: name.clone() };
        diesel::insert_into(product_categories::table)
            .values(&product_category)
            .execute(&self.conn)
            .expect("Error creating product category");
    }

    pub fn list(&self, limit: Option<i32>, offset: Option<i32>) -> Vec<ProductCategory> {
        use super::schema::product_categories::dsl::*;

        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        product_categories
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<ProductCategory>(&self.conn)
            .expect("Error listing product categories")
    }

    pub fn update(&self, _id: i32, new_name: &String) {
        use crate::schema::product_categories::dsl::*;

        diesel::update(product_categories.find(id))
            .set(name.eq(new_name))
            .execute(&self.conn)
            .expect("Error update product category name");
    }

    pub fn delete(&self, _id: i32) {
        use crate::schema::product_categories::dsl::*;

        diesel::delete(product_categories.find(id))
            .execute(&self.conn)
            .expect("Error delete product category");
    }

    #[allow(dead_code)]
    fn delete_all(&self) {
        use crate::schema::product_categories::dsl::*;

        diesel::delete(product_categories)
            .execute(&self.conn)
            .expect("Error delete all product category");
    }
}

#[cfg(test)]
mod test_product_category {
    use crate::establish_connection;
    use crate::services::ProductCategoryService;

    #[test]
    fn test_add() {
        // set up
        let conn = establish_connection();
        let service = ProductCategoryService{conn};
        service.delete_all();

        // test before adding a category, there are no categories
        let categories = service.list(None, None);
        assert_eq!(categories.len(), 0);

        // add a category
        let category_name = String::from("Fruits");
        service.create(&category_name);

        // test one category is in the database
        let categories = service.list(None, None);
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].name, category_name);

        // clean up
        service.delete_all();
    }

    #[test]
    #[should_panic(expected = "Error creating product category")]
    fn test_duplicate_add() {
        // set up
        let conn = establish_connection();
        let service = ProductCategoryService{conn};
        service.delete_all();

        // test the category with the same name cannot be added twice
        let category_name = String::from("Fruits");
        service.create(&category_name);
        service.create(&category_name);

        // clean up
        service.delete_all();
    }

    #[test]
    fn test_update() {
        // set up
        let conn = establish_connection();
        let service = ProductCategoryService{conn};
        service.delete_all();

        // add a category
        let category_name = String::from("Fruits");
        service.create(&category_name);

        // test category is added
        let categories = service.list(None, None);
        let category = &categories[0];
        assert_eq!(category.name, category_name);

        // update category name
        let new_category_name = String::from("Vegetables");
        service.update(category.id, &new_category_name);
        let categories = service.list(None, None);
        assert_eq!(categories[0].name, new_category_name);

        // clean up
        service.delete(category.id);
    }
}