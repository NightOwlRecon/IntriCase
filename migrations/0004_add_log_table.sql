create table logs
(
    id            uuid                     not null
        constraint logs_pk primary key,
    timestamp     timestamp with time zone not null,
    actor         uuid
        constraint logs_actor_fk references users (id),
    target        uuid,
		target_type	  text,
    previous_data text,
    data          text,
    message       text                     not null,
		remote_addr	  inet,
		remote_host	  text,
		remote_port	  integer
);

create index logs_actor_index on logs (actor);

create index logs_target_index on logs (target);

create index logs_target_type_index on logs (target_type);

/* this index is probably redundant while using uuidv7 */
create index logs_timestamp_index on logs (timestamp);

create index logs_remote_addr_index on logs (remote_addr);

create index logs_remote_host_index on logs (remote_host);
