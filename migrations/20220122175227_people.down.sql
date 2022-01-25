-- Add up migration script here
drop index people_first_name_trgm_idx;
drop index people_last_name_trgm_idx;
drop table people;
drop extension if exists pg_trgm;
