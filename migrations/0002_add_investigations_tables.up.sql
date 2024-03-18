create table investigations
(
    id            uuid                     not null
        constraint investigations_pk primary key,
    internal_id   text,
    first_name    text                     not null,
    middle_name   text,
    last_name     text                     not null,
    date_of_birth date                     not null,
    namus_id      text,
    missing_since date                     not null,
    synopsis      text                     not null,
    created       timestamp with time zone not null
);

create index investigations_internal_id_index on investigations (internal_id);

create index investigations_first_name_index on investigations (first_name);

create index investigations_last_name_index on investigations (last_name);

create index investigations_namus_id_index on investigations (namus_id);

create index investigations_created_index on investigations (created);
