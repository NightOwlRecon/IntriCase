create table logs
(
    id            uuid                     not null
        constraint logs_pk primary key,
    actor         uuid
        constraint logs_actor_fk references users (id),
		target        uuid,
		previous_data text,
		data          text,
		message       text										 not null,
    timestamp     timestamp with time zone not null
);

create index logs_actor_index on logs (actor);

create index logs_target_index on logs (target);

/* this index is probably redundant while using uuidv7 */
create index logs_timestamp_index on logs (timestamp);
