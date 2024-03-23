create table questions
(
    id            uuid                     not null
        constraint questions_pk primary key,
		pretty_id			text                     not null,
    summary       text                     not null,
		details       text,
		investigation uuid       			         not null,
		outcome       text,
		creator       uuid                     not null,
		status				text                     not null,
    created       timestamp with time zone not null
);

create index questions_investigation_index on questions (investigation);

create table action_items
(
    id            uuid                     not null
        constraint action_items_pk primary key,
		pretty_id		  text                     not null,
    summary       text                     not null,
    details       text,
		outcome       text,
		assignee      uuid,
		creator       uuid                     not null,
		question      uuid                     not null,
		status				text                     not null,
    assigned      timestamp with time zone not null,
    created       timestamp with time zone not null
);

create index action_items_question_index on action_items (question);
