create table "platoon"
(
	id bigserial primary key,
	team text not null,
	name text not null,
	motto text not null,
	leader_id bigint,
	deputy_leader_id bigint,
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"platoon"');

create table "team"
(
	id bigserial primary key,
	name text not null,
	contact_person_id bigint,
	platoon_id bigint references platoon(id),
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"team"');

create table "platoon_player_without_team"
(
	platoon_id bigint not null references platoon(id),
	player_id bigint not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"platoon_player_without_team"');
