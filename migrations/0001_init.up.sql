create table users
(
    id           uuid not null
                     constraint users_pk
                     primary key,
    email        text not null
                     constraint users_email_unique
                     unique,
    display_name text,
    enabled      boolean not null,
    created      timestamp with time zone not null,
    secret       text,
    otp          text,
    otp_date     timestamp with time zone,
    auth_date    timestamp with time zone
);

create table sessions
(
    id      uuid not null
                constraint sessions_pk
                primary key,
    "user"  uuid not null
                constraint sessions_users_id_fk
                references users,
    created timestamp with time zone not null
);

create index sessions_user_index
    on sessions ("user");

create index sessions_created_index
    on sessions (created);
