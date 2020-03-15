use super::schema::product_categories;

#[derive(Queryable)]
pub struct Brand {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub category_id: i32,
    pub brand_id: i32,
    pub photo_path: Option<String>,
}

#[derive(Insertable)]
#[table_name="product_categories"]
pub struct NewProductCategory<'a> {
    pub name: &'a str,
}