use super::*;
use crate::repository::Repository;
use regex::Regex;

fn match_rule(t: &transaction::Model, r: &rule::Model) -> bool {
    if r.rule_type == "STRING_MATCH" {
        t.description
            .to_lowercase()
            .contains(&r.pattern.to_lowercase())
    } else if r.rule_type == "REGEX" {
        Regex::new(&r.pattern)
            .map(|re| re.is_match(&t.description))
            .unwrap_or_else(|e| {
                println!("Invalid regex: {:?}", e);
                false
            })
    } else {
        println!("Unknown rule type: {}", r.rule_type);
        false
    }
}

pub struct Classifier<'a> {
    repo: &'a Repository,
}

impl<'a> Classifier<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        Self { repo }
    }
}
