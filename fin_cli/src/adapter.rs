use super::*;
use chrono::NaiveDate;
use csv::Reader;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct INGTransaction {
    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Debit")]
    pub amount: f64,
}

impl From<INGTransaction> for CreateTransactionInput {
    fn from(t: INGTransaction) -> Self {
        let date = NaiveDate::parse_from_str(&t.date, "%Y/%m/%d").expect("Failed to parse date");
        let amount_cents = (t.amount * 100.0).floor() as i32;
        CreateTransactionInput {
            date,
            description: t.description,
            amount_cents,
        }
    }
}

pub fn get_ing_transactions(path: &str) -> Vec<CreateTransactionInput> {
    let reader = Reader::from_path(path).expect("Failed to read file");
    reader
        .into_deserialize::<INGTransaction>()
        .map(|result| result.expect("Failed to parse transaction").into())
        .collect()
}
