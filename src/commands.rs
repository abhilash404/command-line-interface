use crate::cli::Command;
use crate::db::Database;
use crate::models::Item;
use crate::error::TodoResult;

pub fn execute_command(command: Command) -> TodoResult<()> {
    let mut db = Database::new()?;

    match command {
        Command::Show { all, completed, incomplete, list_name } => {
            show_tasks(&db, all, completed, incomplete, list_name)?;
        }
        Command::Add { list_name, item } => {
            add_task(&mut db, &list_name, &item)?;
        }
        Command::Complete { list_name, item_number } => {
            complete_task(&mut db, &list_name, item_number)?;
        }
        Command::Incomplete { list_name, item_number } => {
            incomplete_task(&mut db, &list_name, item_number)?;
        }
        Command::Remove { list_name, item_number } => {
            remove_task(&mut db, list_name, item_number)?;
        }
    }

    Ok(())
}

fn show_tasks(
    db: &Database,
    all: bool,
    completed: bool,
    incomplete: bool,
    list_name: Option<String>,
) -> TodoResult<()> {
    let lists = if let Some(name) = list_name {
        vec![db.get_list(&name)?]
    } else {
        db.get_lists()?
    };

    for list in lists {
        println!("List: {}", list.name);
        for (i, item) in list.items.iter().enumerate() {
            if (all || (!completed && !incomplete)) ||
               (completed && item.completed) ||
               (incomplete && !item.completed) {
                println!("  {}. [{}] {}", i + 1, if item.completed { "x" } else { " " }, item.description);
            }
        }
        println!();
    }

    Ok(())
}

fn add_task(db: &mut Database, list_name: &str, item_description: &str) -> TodoResult<()> {
    db.create_list(list_name)?;
    let item = Item {
        description: item_description.to_string(),
        completed: false,
    };
    db.add_item(list_name, item)?;
    println!("Task added to list '{}'", list_name);
    Ok(())
}

fn complete_task(db: &mut Database, list_name: &str, item_number: usize) -> TodoResult<()> {
    db.update_item_status(list_name, item_number, true)?;
    println!("Task {} in list '{}' marked as completed", item_number, list_name);
    Ok(())
}

fn incomplete_task(db: &mut Database, list_name: &str, item_number: usize) -> TodoResult<()> {
    db.update_item_status(list_name, item_number, false)?;
    println!("Task {} in list '{}' marked as incomplete", item_number, list_name);
    Ok(())
}

fn remove_task(
    db: &mut Database,
    list_name: Option<String>,
    item_number: Option<usize>,
) -> TodoResult<()> {
    match (list_name, item_number) {
        (Some(list), Some(item)) => {
            db.remove_item(&list, item)?;
            println!("Task {} removed from list '{}'", item, list);
        }
        (Some(list), None) => {
            db.remove_list(&list)?;
            println!("List '{}' removed", list);
        }
        (None, None) => {
            db.remove_all_lists()?;
            println!("All lists removed");
        }
        _ => println!("Invalid combination of arguments"),
    }
    Ok(())
}