-- Your SQL goes here
create table product (
    id serial primary key,
    name varchar(255) not null,
    created_at timestamptz not null default NOW()
);