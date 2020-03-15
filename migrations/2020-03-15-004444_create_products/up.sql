CREATE TABLE IF NOT EXISTS products (
    id integer primary key not null,
    code varchar unique not null,
    name varchar not null unique,
    category_id integer not null,
    brand_id integer not null,
    photo_path varchar,
    FOREIGN KEY(category_id) REFERENCES product_categories(id),
    FOREIGN KEY(brand_id) REFERENCES brands(id)
);