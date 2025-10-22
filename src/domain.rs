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

impl PrimaryKey for Player {
    fn key(&self) -> Option<i64> {
        self.id
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Team {
    pub id: Option<i64>,
    pub name: String,
    pub contact_person_id: Option<i64>,
    pub platoon_id: Option<i64>,
}

impl PrimaryKey for Team {
    fn key(&self) -> Option<i64> {
        self.id
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Platoon {
    pub id: Option<i64>,
    pub team: String,
    pub name: String,
    pub motto: String,
    pub leader_id: Option<i64>,
    pub deputy_leader_id: Option<i64>,
}

impl PrimaryKey for Platoon {
    fn key(&self) -> Option<i64> {
        self.id
    }
}

#[cfg(feature = "ssr")]
impl From<database::models::Platoon> for Platoon {
    fn from(value: database::models::Platoon) -> Self {
        Self { 
            id: Some(value.id),
            team: value.team,
            name: value.name,
            motto: value.motto,
            leader_id: value.leader_id,
            deputy_leader_id: value.deputy_leader_id,
        }
    }
}

pub trait PrimaryKey {
    fn key(&self) -> Option<i64>;
}
