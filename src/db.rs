use crate::error::{TodoError, TodoResult};
use crate::models::{Item, ItemStatus, List};
use serde_json;
use std::fs;
use std::path::Path;

pub struct Database {
    lists: Vec<List>,
    file_path: String,
}

impl Database {
    pub fn new() -> TodoResult<Self> {
        let file_path = "todo.json".to_string();
        let lists = if Path::new(&file_path).exists() {
            let data = fs::read_to_string(&file_path)?;
            serde_json::from_str(&data)?
        } else {
            Vec::new()
        };
        Ok(Database { lists, file_path })
    }

    fn save(&self) -> TodoResult<()> {
        let data = serde_json::to_string(&self.lists)?;
        fs::write(&self.file_path, data)?;
        Ok(())
    }

    pub fn get_lists(&self) -> TodoResult<Vec<List>> {
        Ok(self.lists.clone())
    }

    pub fn get_list(&self, name: &str) -> TodoResult<List> {
        self.lists
            .iter()
            .find(|list| list.name == name)
            .cloned()
            .ok_or_else(|| TodoError::ListNotFound(name.to_string()))
    }

    pub fn create_list(&mut self, list_name: &str) -> TodoResult<()> {
        if !self.lists.iter().any(|list| list.name == list_name) {
            self.lists.push(List {
                name: list_name.to_string(),
                items: Vec::new(),
            });
            self.save()?;
        }
        Ok(())
    }

    pub fn add_item(&mut self, list_name: &str, item: Item) -> TodoResult<()> {
        let list = self.lists.iter_mut()
            .find(|list| list.name == list_name)
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;
        list.items.push(item);
        self.save()?;
        Ok(())
    }

    pub fn update_item_status(&mut self, list_name: &str, item_number: usize, status: ItemStatus) -> TodoResult<()> {
        let list = self.lists.iter_mut()
            .find(|list| list.name == list_name)
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;
        
        if item_number == 0 || item_number > list.items.len() {
            return Err(TodoError::ItemNotFound(format!("Item {} in list '{}'", item_number, list_name)));
        }

        list.items[item_number - 1].status = status;
        self.save()?;
        Ok(())
    }

    pub fn remove_item(&mut self, list_name: &str, item_number: usize) -> TodoResult<()> {
        let list = self.lists.iter_mut()
            .find(|list| list.name == list_name)
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;
        
        if item_number == 0 || item_number > list.items.len() {
            return Err(TodoError::ItemNotFound(format!("Item {} in list '{}'", item_number, list_name)));
        }

        list.items.remove(item_number - 1);
        self.save()?;
        Ok(())
    }
}