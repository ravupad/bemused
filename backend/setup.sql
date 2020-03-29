create user ravi with encrypted password 'khastgir';
create database bemused;
\c bemused

create table users(
  id bigserial not null primary key ,
  username varchar(32) unique not null,
  password varchar(32) not null);

create table task(
  id bigserial primary key,
  user_id bigint references users(id),
  text varchar(256) not null,
  note varchar(1024) not null,
  category varchar(32) not null,
  schedule_time timestamp with time zone not null,
  schedule_interval_value bigint not null,
  schedule_interval_type varchar(32) not null,
  completed boolean not null);

create table article(
  id varchar(256) not null,
  user_id bigint references users(id),
  title varchar(256) not null,
  text varchar(1024) not null,
  tags varchar(256) not null,
  primary key(id, user_id));

grant all privileges on all tables in schema public to ravi;
grant all privileges on all sequences IN schema public to ravi;
grant all privileges on all functions IN schema public to ravi;
grant all privileges on database bemused to ravi;