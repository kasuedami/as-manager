use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Platoon {
    id: i64,
    pub team: String,
    pub name: String,
    pub motto: String,
    pub leader_id: Option<i64>,
    pub deputy_leader_id: Option<i64>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>
}

impl Platoon {
    pub fn id(&self) -> i64 {
        self.id
    }
}

#[derive(Debug, Serialize)]
pub struct Survey {
    questions: Vec<Question>,   
}

impl Survey {
    pub fn new(questions: Vec<Question>) -> Self {
        Self { questions  }
    }
}

#[derive(Debug, Serialize)]
pub struct Question {
    text: String,
    choices: Vec<Choice>,
}

impl Question {
    pub fn new(text: String, choices: Vec<Choice>) -> Self {
        Self { text, choices }
    }
}

#[derive(Debug, Serialize)]
pub struct Choice {
    text: String,
}

impl Choice {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
