CREATE TABLE IF NOT EXISTS product_categories (
    id integer primary key not null,
    name varchar not null unique
);