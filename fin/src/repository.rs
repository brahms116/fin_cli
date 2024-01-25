use chrono::NaiveDate;
use futures::stream::{FuturesUnordered, Stream, StreamExt, TryStreamExt};

use super::*;
use sea_orm::{error::DbErr, DatabaseConnection, Set};
use sqlx::error::{Error, ErrorKind};

pub struct Repository {
    db: DatabaseConnection,
}

#[derive(Debug)]
pub struct TransactionUpdate {
    pub transaction_id: String,
    pub category_id: String,
}

pub struct CreateTransactionModel {
    pub id: String,
    pub amount_cents: i32,
    pub description: String,
    pub date: NaiveDate,
    pub category: Option<String>,
}

#[derive(Debug)]
pub enum CreateTransactionResultMessage {
    Success,
    Duplicate,
    Error(String),
}

impl From<&DbErr> for CreateTransactionResultMessage {
    fn from(e: &DbErr) -> Self {
        match e {
            DbErr::Query(RuntimeErr::SqlxError(Error::Database(x))) => {
                if x.kind() == ErrorKind::UniqueViolation {
                    CreateTransactionResultMessage::Duplicate
                } else {
                    CreateTransactionResultMessage::Error(format!("{:?}", x))
                }
            }
            _ => CreateTransactionResultMessage::Error(format!("{:?}", e)),
        }
    }
}

#[derive(Debug)]
pub struct CreateTransactionResult {
    pub description: String,
    pub date: NaiveDate,
    pub message: CreateTransactionResultMessage,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_rules(&self) -> Result<Vec<rule::Model>, StringErr> {
        Ok(rule::Entity::find().all(&self.db).await?)
    }

    pub async fn get_uncategorised_transactions(
        &self,
    ) -> Result<impl Stream<Item = Result<transaction::Model, StringErr>> + Send + '_, StringErr>
    {
        Ok(Transaction::find()
            .filter(transaction::Column::Category.is_null())
            .stream(&self.db)
            .await?
            .map_err(|e| e.into()))
    }

    pub async fn get_transactions(
        &self,
    ) -> Result<impl Stream<Item = Result<transaction::Model, StringErr>> + Send + '_, StringErr>
    {
        Ok(Transaction::find()
            .stream(&self.db)
            .await?
            .map_err(|e| e.into()))
    }

    pub async fn apply_update(&self, u: TransactionUpdate) -> Result<(), StringErr> {
        let _ = transaction::ActiveModel {
            id: Set(u.transaction_id),
            category: Set(Some(u.category_id)),
            ..Default::default()
        }
        .update(&self.db)
        .await?;
        Ok(())
    }

    pub async fn apply_updates(&self, us: Vec<TransactionUpdate>) -> Result<(), StringErr> {
        let fut: FuturesUnordered<_> = us.into_iter().map(|u| self.apply_update(u)).collect();
        Ok(fut
            .try_for_each_concurrent(10, |_| async { Ok(()) })
            .await?)
    }

    pub async fn create_transaction(&self, t: CreateTransactionModel) -> CreateTransactionResult {
        let r = transaction::ActiveModel {
            id: Set(t.id),
            amount_cents: Set(t.amount_cents),
            description: Set(t.description.clone()),
            date: Set(t.date),
            category: Set(t.category),
            ..Default::default()
        }
        .insert(&self.db)
        .await;

        let mes = match r {
            Ok(_) => CreateTransactionResultMessage::Success,
            Err(e) => (&e).into(),
        };

        CreateTransactionResult {
            description: t.description,
            date: t.date,
            message: mes,
        }
    }

    pub async fn create_transactions(
        &self,
        ts: Vec<CreateTransactionModel>,
    ) -> Vec<CreateTransactionResult> {
        let fut: FuturesUnordered<_> = ts.into_iter().map(|t| self.create_transaction(t)).collect();
        fut.collect().await
    }
}
