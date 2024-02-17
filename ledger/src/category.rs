use std::{collections::{HashMap, HashSet}, path::Iter};

#[derive(Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct CategoryId(String);

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionCategory {
    id: CategoryId,
    is_also: HashSet<CategoryId>, // Associate a transaction with other categories
}

impl TransactionCategory {
    pub fn name(&self) -> &String {
        &self.id.0
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionCategories {
    category_set: HashMap<CategoryId, TransactionCategory>, 
}

impl TransactionCategories {
    pub fn new_empty() -> TransactionCategories  {
        TransactionCategories { category_set: HashMap::new() }
    }

    pub fn categories(&self) -> impl Iterator<Item = &TransactionCategory> {
        self.category_set.values().into_iter()
    }
}