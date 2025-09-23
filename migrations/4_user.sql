create table "player"
(
	id bigserial primary key,
	email text not null,
	password_hash bytea not null unique,
	tag_name text not null,
	active boolean not null,
	team_id bigint references team(id) default null,
	created_at timestamptz not null default now(),
	updated_at timestamptz
);

select trigger_updated_at('"player"');

alter table "platoon"
add constraint player_leader_fk
foreign key (leader_id)
references player(id);

alter table "platoon"
add constraint player_deputy_leader_fk
foreign key (deputy_leader_id)
references player(id);

alter table "team"
add constraint player_contact_person_fk
foreign key (contact_person_id)
references player(id);

alter table "platoon_player_without_team"
add constraint player_id_fk
foreign key (player_id)
references player(id);
