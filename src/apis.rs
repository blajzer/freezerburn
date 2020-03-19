use actix_web::{delete, get, HttpRequest, HttpResponse, post, Responder, web};
use serde;

use super::establish_connection;
use super::models::ProductCategory;
use super::services::ProductCategoryService;

#[derive(serde::Serialize)]
struct ListResponsePayload {
    pub limit: i32,
    pub offset: i32,
    pub data: Vec<ProductCategory>,
}

#[derive(serde::Deserialize)]
pub struct CreateRequestPayload {
    pub name: String,
}

#[get("/api/productCategories/")]
pub async fn list_product_categories() -> impl Responder {
    let limit: i32 = 100;
    let offset: i32 = 0;

    let conn = establish_connection();
    let service = ProductCategoryService{conn};
    let categories = service.list(Some(limit), Some(offset));
    let response_payload = ListResponsePayload {limit, offset, data: categories};

    HttpResponse::Ok().json(response_payload)
}

#[post("/api/productCategories/")]
pub async fn create_product_category(payload: web::Json<CreateRequestPayload>) -> impl Responder {
    let conn = establish_connection();
    let service = ProductCategoryService{conn};
    let result = service.create(&payload.name);

    match result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::UnprocessableEntity()
    }
}

#[delete("/api/productCategories/{productCategoryId}/")]
pub async fn delete_product_category(request: HttpRequest) -> impl Responder {
    let product_category_id: i32 = request
        .match_info()
        .query("productCategoryId")
        .parse()
        .unwrap();

    let conn = establish_connection();
    let service = ProductCategoryService{conn};
    service.delete(product_category_id);

    HttpResponse::NoContent()
}
