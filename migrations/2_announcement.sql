create table "announcement"
(
	id bigserial primary key,
	title text not null,
	content text not null,
	hidden boolean not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"announcement"');
