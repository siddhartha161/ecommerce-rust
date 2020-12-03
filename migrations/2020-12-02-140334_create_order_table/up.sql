-- Your SQL goes here
CREATE TABLE orders (
   id serial primary key,
   email varchar(255) not null,
   created_at timestamptz not null default NOW()
 );