create table people (
    id text not null primary key,
    first_name text not null,
    last_name text not null
);

create virtual table people_fts using fts5(
    first_name,
    last_name,
    id UNINDEXED,
    content='people',
    content_rowid='rowid'
);

create trigger people_after_insert after insert on people
begin
    insert into people_fts
        (rowid, first_name, last_name)
    values
        (new.rowid, new.first_name, new.last_name);
end;

create trigger people_after_delete after delete on people
begin
    insert into people_fts
        (people_fts, rowid, first_name, last_name)
    values
        ('delete', old.rowid, old.first_name, old.last_name);
end;

create trigger people_after_update AFTER UPDATE ON people
begin
    insert into people_fts
        (people_fts, rowid, first_name, last_name)
    values
        ('delete', old.rowid, old.first_name, old.last_name);

    insert into people_fts
        (rowid, first_name, last_name)
    values
        (new.rowid, new.first_name, new.last_name);
end;

create index people_last_name_first_name_idx on people(last_name, first_name);