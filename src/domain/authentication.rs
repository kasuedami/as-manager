use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use crypto_hashes::sha3::{Digest, Sha3_512};
use serde::Deserialize;
use sqlx::PgPool;

use super::Player;

#[derive(Clone)]
pub struct Backend {
    pool: PgPool,
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
    pub redirect_to: String,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthenticationError {
    #[error("user not found")]
    UserNotFound,
    #[error("password for user not correct")]
    PasswordNotCorrect,
    #[error("error from database")]
    DatabaseError,
}

impl Backend {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = Player;
    type Credentials = Credentials;
    type Error = AuthenticationError;

    async fn authenticate(&self, Credentials { email, password, .. }: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let mut hasher = Sha3_512::default();
        hasher.update(password);
        let password_hash = format!("{:x}", hasher.finalize());

        let player = sqlx::query_as::<_, Player>("select * from player where email like $1")
            .bind(&email)
            .fetch_optional(&self.pool)
            .await;

        match player {
            Ok(player) => {
                if let Some(player) = player {
                    if player.password_hash == password_hash.into_bytes() {
                        Ok(Some(player))
                    } else {
                        Err(AuthenticationError::PasswordNotCorrect)
                    }
                } else {
                    Err(AuthenticationError::UserNotFound)
                }
            },
            Err(_) => {
                Err(AuthenticationError::DatabaseError)
            }
        }
    }

    async fn get_user(&self, player_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let player = sqlx::query_as::<_, Player>("select * from player where id = $1")
            .bind(player_id)
            .fetch_optional(&self.pool)
            .await;

        match player {
            Ok(player) => {
                Ok(player)
            },
            Err(_) => {
                Err(AuthenticationError::DatabaseError)
            },
        }
    }
}

impl AuthUser for Player {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
