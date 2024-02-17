use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct CategoryId(String);

impl From<String> for CategoryId {
    fn from(value: String) -> Self {
        CategoryId(value)
    }
}

impl Display for CategoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionCategory {
    id: CategoryId,
}

impl TransactionCategory {
    // TODO allow for a separate name
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

    pub fn create_category(&mut self, id: String) -> Result<(), String> {
        let id = CategoryId(id);
        if self.category_set.contains_key(&id) {
            return Err(format!("Category {} already exists", id.0));
        }

        self.category_set.insert(id.clone(), TransactionCategory {id});

        Ok(())
    }

    pub fn categories(&self) -> impl Iterator<Item = &TransactionCategory> {
        self.category_set.values()
    }

    pub fn get_category(&self, id: &CategoryId) -> Option<&TransactionCategory> {
        self.category_set.get(id)
    }
}