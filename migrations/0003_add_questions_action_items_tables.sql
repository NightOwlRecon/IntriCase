create table questions
(
    id            uuid                     not null
        constraint questions_pk primary key,
    created       timestamp with time zone not null,
    creator       uuid                     not null
        constraint questions_creator_fk references users (id),
    pretty_id     text                     not null,
    summary       text                     not null,
    details       text,
    investigation uuid                     not null,
    outcome       text,
    status        text                     not null
);

create index questions_investigation_index on questions (investigation);

create table action_items
(
    id        uuid                     not null
        constraint action_items_pk primary key,
    created   timestamp with time zone not null,
    creator   uuid                     not null
        constraint action_items_creator_fk references users (id),
    pretty_id text                     not null,
    summary   text                     not null,
    details   text,
    outcome   text,
    assignee  uuid
        constraint action_items_assignee_fk references users (id),
    question  uuid                     not null
        constraint action_items_question_fk references questions (id),
    status    text                     not null,
    assigned  timestamp with time zone,
    resolved  timestamp with time zone
);

create index action_items_assignee_index on action_items (assignee);
create index action_items_question_index on action_items (question);
