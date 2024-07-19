use std::fs;
use std::path::PathBuf;
use serde_json;
use crate::models::{List, Item};
use crate::error::{TodoError, TodoResult};

pub struct Database {
    file_path: PathBuf,
    lists: Vec<List>,
}

impl Database {
    pub fn new() -> TodoResult<Self> {
        let file_path = PathBuf::from("todo_lists.json");
        let lists = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };
        Ok(Self { file_path, lists })
    }

    fn save(&self) -> TodoResult<()> {
        let content = serde_json::to_string_pretty(&self.lists)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn get_lists(&self) -> TodoResult<Vec<List>> {
        Ok(self.lists.clone())
    }

    pub fn get_list(&self, name: &str) -> TodoResult<List> {
        self.lists.iter()
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

    pub fn update_item_status(&mut self, list_name: &str, item_number: usize, completed: bool) -> TodoResult<()> {
        let list = self.lists.iter_mut()
            .find(|list| list.name == list_name)
            .ok_or_else(|| TodoError::ListNotFound(list_name.to_string()))?;
        
        if item_number == 0 || item_number > list.items.len() {
            return Err(TodoError::ItemNotFound(format!("Item {} in list '{}'", item_number, list_name)));
        }

        list.items[item_number - 1].completed = completed;
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

    pub fn remove_list(&mut self, list_name: &str) -> TodoResult<()> {
        self.lists.retain(|list| list.name != list_name);
        self.save()?;
        Ok(())
    }

    pub fn remove_all_lists(&mut self) -> TodoResult<()> {
        self.lists.clear();
        self.save()?;
        Ok(())
    }
}