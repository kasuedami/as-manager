use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::database;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Player {
    pub id: Option<i64>,
    pub email: String,
    pub tag_name: String,
    pub active: bool,
    pub team_id: Option<i64>,
}

#[cfg(feature = "ssr")]
impl From<database::models::Player> for Player {
    fn from(value: database::models::Player) -> Self {
        Self {
            id: Some(value.id),
            email: value.email,
            tag_name: value.tag_name,
            active: value.active,
            team_id: value.team_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Team {
    pub id: Option<i64>,
    pub name: String,
    pub contact_person_id: Option<i64>,
    pub platoon_id: Option<i64>,
}

#[cfg(feature = "ssr")]
impl From<database::models::Team> for Team {
    fn from(value: database::models::Team) -> Self {
        Self { 
            id: Some(value.id),
            name: value.name,
            contact_person_id: value.contact_person_id,
            platoon_id: value.platoon_id,
        }
    }
}
