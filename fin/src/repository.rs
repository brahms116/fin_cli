use futures::stream::{FuturesUnordered, Stream, TryStreamExt};

use super::*;
use sea_orm::{DatabaseConnection, Set};

pub struct Repository {
    db: DatabaseConnection,
}

pub struct TransactionUpdate {
    pub transaction_id: String,
    pub category_id: String,
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
}
