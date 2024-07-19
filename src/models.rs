use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct List {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub description: String,
    pub status: ItemStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ItemStatus {
    Incomplete,
    Working,
    Completed,
}