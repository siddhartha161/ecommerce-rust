-- Your SQL goes here

 CREATE TABLE line_item (
   id serial primary key,
   orders int REFERENCES orders ON DELETE CASCADE,
   product int not null REFERENCES product ON DELETE CASCADE,
   quantity int not null
 );