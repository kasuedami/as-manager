create table "platoons"
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

select trigger_updated_at('"platoons"');

create table "teams"
(
	id bigserial primary key,
	name text not null,
	contact_person_id bigint,
	platoon_id bigint references platoons(id),
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"teams"');

create table "platoon_player_without_team"
(
	platoon_id bigint not null references platoons(id),
	player_id bigint not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz,
	primary key (platoon_id, player_id)
);

select trigger_updated_at('"platoon_player_without_team"');

create table "players"
(
	id bigserial primary key,
	email text not null,
	tag_name text not null,
	active boolean not null,
	team_id bigint references teams(id) default null,
	password_hash bytea not null,
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"players"');

alter table "platoons"
add constraint player_leader_fk
foreign key (leader_id)
references players(id);

alter table "platoons"
add constraint player_deputy_leader_fk
foreign key (deputy_leader_id)
references players(id);

alter table "teams"
add constraint player_contact_person_fk
foreign key (contact_person_id)
references players(id);

alter table "platoon_player_without_team"
add constraint player_id_fk
foreign key (player_id)
references players(id);
