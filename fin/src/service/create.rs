use chrono::NaiveDate;

use super::*;

pub struct CreateTransactionInput {
    pub amount_cents: i32,
    pub description: String,
    pub date: NaiveDate,
}

impl Service {
    pub async fn create_transactions(
        &self,
        is: Vec<CreateTransactionInput>,
    ) -> Vec<CreateTransactionResult> {
        let ms = is.into_iter().map(input_to_model).collect::<Vec<_>>();
        self.repo.create_transactions(ms).await
    }
}

fn input_to_model(i: CreateTransactionInput) -> CreateTransactionModel {
    let id = format!("{}{}", i.date, i.description);
    // Skip classifying for now
    let category = None;
    CreateTransactionModel {
        id,
        amount_cents: i.amount_cents,
        description: i.description,
        date: i.date,
        category,
    }
}
