mod iterator;

use futures::stream::{FuturesUnordered, Stream, StreamExt};
pub use iterator::*;

use super::*;
use sea_orm::{DatabaseConnection, Set};

pub struct Repository {
    db: DatabaseConnection,
    page_size: u64,
}

pub struct TransactionUpdate {
    pub transaction_id: String,
    pub category_id: String,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db, page_size: 500 }
    }

    pub fn new_with_page_size(db: DatabaseConnection, page_size: u64) -> Self {
        Self { db, page_size }
    }

    pub async fn get_uncategoriesd_transactions_stream(
        &self,
    ) -> impl Stream<Item = Result<transaction::Model, DbErr>> + Send + '_ {
        Transaction::find()
            .filter(transaction::Column::Category.is_null())
            .stream(&self.db)
            .await
            .expect("What the heck")
    }

    pub async fn get_transactions_stream(
        &self,
    ) -> impl Stream<Item = Result<transaction::Model, DbErr>> + Send + '_ {
        Transaction::find()
            .stream(&self.db)
            .await
            .expect("What the heck")
    }

    pub async fn apply_update(&self, u: TransactionUpdate) -> () {
        transaction::ActiveModel {
            id: Set(u.transaction_id),
            category: Set(Some(u.category_id)),
            ..Default::default()
        }
        .update(&self.db)
        .await
        .unwrap();
    }

    pub async fn apply_updates(&self, us: Vec<TransactionUpdate>) -> () {
        let fut: FuturesUnordered<_> = us.into_iter().map(|u| self.apply_update(u)).collect();
        fut.for_each_concurrent(10, |_| async {}).await;
    }
}
