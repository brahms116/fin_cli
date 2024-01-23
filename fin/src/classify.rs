use super::*;
use crate::repository::{Repository, TransactionUpdate};
use futures::{StreamExt, TryStream, TryStreamExt};
use regex::{Error as RegexError, Regex};

use sea_orm::Database;
pub struct Classifier {
    repo: Repository,
}

impl From<RegexError> for StringErr {
    fn from(e: RegexError) -> Self {
        StringErr::new(format!("{:?}", e))
    }
}

impl Classifier {
    pub async fn new(database_url: &str) -> Result<Self, StringErr> {
        Ok(Database::connect(database_url).await.map(|db| Self {
            repo: Repository::new(db),
        })?)
    }

    pub async fn classify_all_transactions(&self) -> Result<Vec<TransactionUpdate>, StringErr> {
        let ts = self.repo.get_transactions().await?;
        let rs = self.repo.get_rules().await?;
        classify_transactions(ts, rs).await
    }

    pub async fn classify_uncategorised_transactions(
        &self,
    ) -> Result<Vec<TransactionUpdate>, StringErr> {
        let ts = self.repo.get_uncategorised_transactions().await?;
        let rs = self.repo.get_rules().await?;
        classify_transactions(ts, rs).await
    }

    pub async fn update_transactions(
        &self,
        updates: Vec<TransactionUpdate>,
    ) -> Result<(), StringErr> {
        self.repo.apply_updates(updates).await
    }
}

async fn classify_transactions<T>(
    ts: T,
    rs: Vec<rule::Model>,
) -> Result<Vec<TransactionUpdate>, StringErr>
where
    T: TryStream<Item = Result<transaction::Model, StringErr>>,
{
    let map_f = |rt: Result<transaction::Model, StringErr>| rt.map(|t| match_rules(&t, &rs));

    ts.map(map_f)
        .try_filter_map(|ou| async move { Ok(ou) })
        .try_collect()
        .await
}

fn match_rule(t: &transaction::Model, r: &rule::Model) -> Result<bool, StringErr> {
    if r.rule_type == "STRING_MATCH" {
        Ok(t.description
            .to_lowercase()
            .contains(&r.pattern.to_lowercase()))
    } else if r.rule_type == "REGEX" {
        Ok(Regex::new(&r.pattern).map(|re| re.is_match(&t.description))?)
    } else {
        Err(StringErr::new(format!(
            "Unknown rule type: {}",
            r.rule_type
        )))
    }
}

fn match_rules(t: &transaction::Model, rs: &[rule::Model]) -> Option<TransactionUpdate> {
    rs.iter()
        .find(|r| {
            match_rule(t, r).unwrap_or_else(|e| {
                println!("Error matching rule {}: {}", r.id, e);
                false
            })
        })
        .map(|r| TransactionUpdate {
            transaction_id: t.id.clone(),
            category_id: r.category.clone(),
        })
}
