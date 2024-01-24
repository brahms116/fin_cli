use super::*;

impl From<RegexError> for StringErr {
    fn from(e: RegexError) -> Self {
        StringErr::new(format!("{:?}", e))
    }
}

impl Service {
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

    pub async fn apply_transaction_updates(
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
    let find_f = |r: &&rule::Model| {
        match_rule(t, r).unwrap_or_else(|e| {
            println!("Error matching rule {}: {}", r.id, e);
            false
        })
    };

    let handle_match = |r: &rule::Model| {
        if let Some(r2) = &t.category {
            if r2 == &r.category {
                return None;
            }
        }
        Some(TransactionUpdate {
            transaction_id: t.id.clone(),
            category_id: r.category.clone(),
        })
    };

    rs.iter().find(find_f).and_then(handle_match)
}
