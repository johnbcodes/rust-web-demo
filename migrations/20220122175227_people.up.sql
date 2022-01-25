-- Add up migration script here
create extension if not exists pg_trgm;

create table people (
    id text not null primary key,
    first_name text not null,
    last_name text not null
);

create index people_first_name_trgm_idx on people using gin (first_name gin_trgm_ops);
create index people_last_name_trgm_idx on people using gin (last_name gin_trgm_ops);