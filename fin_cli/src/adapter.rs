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

#[derive(Deserialize, Debug)]
pub struct CustomTransaction {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Debt")]
    pub amount: f64,
}

impl From<CustomTransaction> for CreateTransactionInput {
    fn from(t: CustomTransaction) -> Self {
        let date = NaiveDate::parse_from_str(&t.date, "%m/%d/%Y").expect("Failed to parse date");
        let amount_cents = (t.amount * 100.0).floor() as i32;
        CreateTransactionInput {
            date,
            description: t.description,
            amount_cents,
        }
    }
}

pub fn get_custom_transactions(path: &str) -> Vec<CreateTransactionInput> {
    let reader = Reader::from_path(path).expect("Failed to read file");
    reader
        .into_deserialize::<CustomTransaction>()
        .map(|result| result.expect("Failed to parse transaction").into())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_custom_transactions() {
        let path = "fixtures/custom_transactions.csv";
        let expected = vec![
            CreateTransactionInput {
                date: NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
                description: "Myer".to_string(),
                amount_cents: -5000,
            },
            CreateTransactionInput {
                date: NaiveDate::from_ymd_opt(2023, 1, 24).unwrap(),
                description: "Test Buggee".to_string(),
                amount_cents: -2000,
            },
        ];
        let actual = get_custom_transactions(path);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ing_transactions() {
        let path = "fixtures/ing_transactions.csv";
        let expected = vec![
            CreateTransactionInput {
                date: NaiveDate::from_ymd_opt(2024, 1, 24).unwrap(),
                description: "Some buy hey?".to_string(),
                amount_cents: -4800,
            },
            CreateTransactionInput {
                date: NaiveDate::from_ymd_opt(2023, 12, 12).unwrap(),
                description: "Credit".to_string(),
                amount_cents: 3000,
            },
        ];
        let actual = get_ing_transactions(path);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bendigo_transactions() {
        let path = "fixtures/bendigo_transactions.csv";
        let expected = vec![
            CreateTransactionInput {
                date: NaiveDate::from_ymd_opt(2024, 1, 14).unwrap(),
                description: "My loss".to_string(),
                amount_cents: -40000,
            },
            CreateTransactionInput {
                date: NaiveDate::from_ymd_opt(2023, 12, 2).unwrap(),
                description: "My earn".to_string(),
                amount_cents: 100000,
            },
        ];
        let actual = get_bendigo_transactions(path);
        assert_eq!(expected, actual);
    }
}
