mod classify;

use super::*;
use crate::repository::{Repository, TransactionUpdate};
use futures::{StreamExt, TryStream, TryStreamExt};
use regex::{Error as RegexError, Regex};

use sea_orm::Database;

pub use classify::*;

pub struct Service {
    repo: Repository,
}

impl Service {
    pub async fn new(database_url: &str) -> Result<Self, StringErr> {
        Ok(Database::connect(database_url).await.map(|db| Self {
            repo: Repository::new(db),
        })?)
    }

    pub fn from_repo(repo: Repository) -> Self {
        Self { repo }
    }
}
