create table "platoon_survey"
(
	id bigserial primary key,
	platoon_id bigint not null references platoon(id)
);

create table "platoon_survey_question"
(
	id bigserial primary key,
	text text not null,
	survey_id bigint not null references platoon_survey(id)
);

create table "platoon_survey_question_option"
(
	id bigserial primary key,
	text text not null,
	question_id bigint not null references platoon_survey_question(id)
);

create table "platoon_survey_answer"
(
	id bigserial primary key,
	player_id bigint not null references player(id),
	survey_id bigint not null references platoon_survey(id),
	question_id bigint not null references platoon_survey_question(id),
	option_id bigint not null references platoon_survey_question_option(id)
);

create table "team_survey"
(
	id bigserial primary key,
	team_id bigint not null references team(id)
);

create table "team_survey_question"
(
	id bigserial primary key,
	text text not null,
	survey_id bigint not null references team_survey(id)
);

create table "team_survey_question_option"
(
	id bigserial primary key,
	text text not null,
	question_id bigint not null references team_survey_question(id)
);

create table "team_survey_answer"
(
	id bigserial primary key,
	player_id bigint not null references player(id),
	survey_id bigint not null references team_survey(id),
	question_id bigint not null references team_survey_question(id),
	option_id bigint not null references team_survey_question_option(id)
);
