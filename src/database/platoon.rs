pub mod survey {
    use serde::Serialize;
    use sqlx::{FromRow, PgPool};

    use crate::domain;

    pub async  fn all_surveys_for_platoon_id(platoon_id: i64, pool: &PgPool) -> Vec<domain::platoon::Survey> {
        let surveys = sqlx::query_as::<_, Survey>("select * from platoon_survey where platoon_id = $1")
            .bind(platoon_id)
            .fetch_all(pool)
            .await
            .unwrap();
        
        let survey_ids: Vec<_> = surveys.iter().map(|survey| survey.id).collect();

        let questions = sqlx::query_as::<_, Question>("select * from platoon_survey_question where survey_id = any($1)")
            .bind(survey_ids)
            .fetch_all(pool)
            .await
            .unwrap();

        let question_ids: Vec<_> = questions.iter().map(|question| question.id).collect();

        let choices = sqlx::query_as::<_, Choice>("select * from platoon_survey_question_option where question_id = any($1)")
            .bind(question_ids)
            .fetch_all(pool)
            .await
            .unwrap();

        surveys.iter().map(|survey| {
            let questions = questions.iter().filter_map(|question| {
                if question.survey_id == survey.id {
                    let choices = choices.iter().filter_map(|choice| {
                        if choice.question_id == question.id {
                            Some(domain::platoon::Choice::new(choice.text.clone()))
                        } else {
                            None
                        }
                    }).collect();

                    Some(domain::platoon::Question::new(question.text.clone(), choices))
                } else {
                    None
                }
            }).collect();

            domain::platoon::Survey::new(questions)
        }).collect()
    }

    #[derive(Debug, Serialize, FromRow)]
    pub struct Survey {
        id: i64,
        platoon_id: i64,
    }
    
    impl Survey {
        pub fn id(&self) -> i64 {
            self.id
        }
    }

    #[derive(Debug, Serialize, FromRow)]
    pub struct Question {
        id: i64,
        text: String,
        survey_id: i64,
    }

    impl Question {
        pub fn id(&self) -> i64 {
            self.id
        }
    }

    #[derive(Debug, Serialize, FromRow)]
    pub struct Choice {
        id: i64,
        text: String,
        question_id: i64,
    }

    impl Choice {
        pub fn id(&self) -> i64 {
            self.id
        }
    }

    #[derive(Debug, Serialize, FromRow)]
    pub struct Answer {
        id: i64,
        player_id: i64,
        survey_id: i64,
        question_id: i64,
        option_id: i64,
    }

    impl Answer {
        pub fn id(&self) -> i64 {
            self.id
        }
    }
}
