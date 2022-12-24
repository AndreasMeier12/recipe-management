create table ingredient_dg_tmp
(
    id         INTEGER
        primary key autoincrement,
    name       VARCHAR(255)
        unique,
    created_at REAL default (datetime('now', 'localtime'))
);

insert into ingredient_dg_tmp(id, name, created_at)
select id, name, created_at
from ingredient;

drop table ingredient;

alter table ingredient_dg_tmp
    rename to ingredient;

