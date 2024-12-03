create sequence products_id_seq;
create sequence users_id_seq;
create sequence orders_id_seq;
create sequence download_verifications_id_seq;

create table products (
    id text primary key,
    name text,
    price_in_cents integer,
    file_path text,
    image_path text,
    description text,
    is_available_for_purchase boolean default true,
    created_at bigint not null,
    updated_at bigint
);

create table users (
    id text primary key,
    email text unique,
    created_at bigint not null,
    updated_at bigint
);

create table orders (
    id text primary key,
    user_id text not null references users(id) on delete cascade,
    product_id text not null references products(id) on delete restrict,
    price_in_cents integer,
    created_at bigint not null,
    updated_at bigint
);

create table download_verifications (
    id text primary key,
    product_id text not null references products(id) on delete cascade,
    expires_at bigint,
    created_at bigint not null
);
