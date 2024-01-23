use super::*;
use crate::repository::{Repository, TransactionUpdate};
use futures::{StreamExt, TryStream, TryStreamExt};
use regex::Regex;

fn match_rule(t: &transaction::Model, r: &rule::Model) -> Result<bool, StringErr> {
    if r.rule_type == "STRING_MATCH" {
        Ok(t.description
            .to_lowercase()
            .contains(&r.pattern.to_lowercase()))
    } else if r.rule_type == "REGEX" {
        Regex::new(&r.pattern)
            .map(|re| re.is_match(&t.description))
            .map_err(|e| StringErr::new(format!("Invalid regex for rule {}: {}", r.id, e)))
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
                println!("Error matching: {}", e);
                false
            })
        })
        .map(|r| TransactionUpdate {
            transaction_id: t.id.clone(),
            category_id: r.category.clone(),
        })
}

pub struct Classifier<'a> {
    repo: &'a Repository,
}

impl<'a> Classifier<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        Self { repo }
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
