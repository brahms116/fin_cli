use super::*;
use chrono::NaiveDate;
use csv::{Reader, ReaderBuilder};
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct INGTransaction {
    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Credit")]
    pub credit: Option<f64>,

    #[serde(rename = "Debit")]
    pub debit: Option<f64>,
}

impl From<INGTransaction> for CreateTransactionInput {
    fn from(t: INGTransaction) -> Self {
        let date = NaiveDate::parse_from_str(&t.date, "%d/%m/%Y").expect("Failed to parse date");
        let amount = t.credit.unwrap_or(t.debit.unwrap_or(0.0));
        let amount_cents = (amount * 100.0).floor() as i32;
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

#[derive(Deserialize, Debug)]
pub struct BendigoTransaction {
    pub date: String,
    pub amount: f64,
    pub description: String,
}

impl From<BendigoTransaction> for CreateTransactionInput {
    fn from(t: BendigoTransaction) -> Self {
        let date = NaiveDate::parse_from_str(&t.date, "%d/%m/%Y").expect("Failed to parse date");
        let amount_cents = (t.amount * 100.0).floor() as i32;
        CreateTransactionInput {
            date,
            description: t.description,
            amount_cents,
        }
    }
}

pub fn get_bendigo_transactions(path: &str) -> Vec<CreateTransactionInput> {
    let reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .expect("Failed to read file");
    reader
        .into_deserialize::<BendigoTransaction>()
        .map(|result| result.expect("Failed to parse transaction").into())
        .collect()
}
