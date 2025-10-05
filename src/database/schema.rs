// @generated automatically by Diesel CLI.

diesel::table! {
    platoon_player_without_team (platoon_id, player_id) {
        platoon_id -> Int8,
        player_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    platoons (id) {
        id -> Int8,
        team -> Text,
        name -> Text,
        motto -> Text,
        leader_id -> Nullable<Int8>,
        deputy_leader_id -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    players (id) {
        id -> Int8,
        email -> Text,
        tag_name -> Text,
        active -> Bool,
        team_id -> Nullable<Int8>,
        password_hash -> Bytea,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    teams (id) {
        id -> Int8,
        name -> Text,
        contact_person_id -> Nullable<Int8>,
        platoon_id -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(platoon_player_without_team -> platoons (platoon_id));
diesel::joinable!(platoon_player_without_team -> players (player_id));
diesel::joinable!(teams -> platoons (platoon_id));

diesel::allow_tables_to_appear_in_same_query!(
    platoon_player_without_team,
    platoons,
    players,
    teams,
);
