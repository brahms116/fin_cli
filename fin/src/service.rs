mod classify;
mod create;

use super::*;
use crate::repository::{CreateTransactionResult, Repository, TransactionUpdate, CreateTransactionModel};
use futures::{StreamExt, TryStream, TryStreamExt};
use regex::{Error as RegexError, Regex};

use sea_orm::Database;

pub use classify::*;
pub use create::*;

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
