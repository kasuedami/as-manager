create table "events"
(
	id bigserial primary key,
	name text not null,
	description text not null,
	start_time timestamptz not null,
	end_time timestamptz,
	creator bigint references players(id),
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"events"');

create table "event_members"
(
	event_id bigserial not null references events(id),
	player_id bigserial not null references players(id),
	created_at timestamptz not null default now(),
	primary key (event_id, player_id)
);
