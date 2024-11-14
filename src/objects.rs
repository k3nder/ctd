use std::collections::HashMap;
use std::string::ToString;
use kommons_macros::write_file;
use prettytable::{Cell, Row, Table};
use prettytable::color::Color;
use serde::{Deserialize, Serialize};
use toml::de::Error;
use once_cell::sync::Lazy;

static USERNAME: Lazy<String> = Lazy::new(|| whoami::username());
pub static TODO_FILE_LOC: Lazy<String> = Lazy::new(|| format!("/home/{}/.config/todo.toml", USERNAME.to_string()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToDo {
    pub title: String,
    pub description: String,
    pub done: bool,
    pub priority: i32,
    pub limit: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub to_do: HashMap<String, ToDo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToDoFile {
    pub to_do: HashMap<String, ToDo>,
    pub projects: HashMap<String, Project>
}
impl ToDoFile {
    pub fn deserialize(input: &str) -> Result<Self, Error> {
        toml::from_str(input)
    }
    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(&self)
    }
    pub fn save(&self) {
        let serialized = self.serialize().unwrap();
        //println!("saving: {:?}", serialized);
        write_file!(TODO_FILE_LOC.clone(), serialized);
    }
    pub fn print_tasks(&self) {
        // Crear la tabla
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Title"),
            Cell::new("Description"),
            Cell::new("Priority"),
            Cell::new("To Limit"),
            Cell::new("Done"),
        ]));

        for (name, todo) in self.to_do.iter() {
                table.add_row(Row::new(vec![
                    Cell::new(&name),
                    Cell::new(&todo.description),
                    Cell::new(&todo.priority.to_string()),
                    Cell::new(&todo.limit.clone().unwrap_or("Undefined".to_string())),
                    Cell::new(&todo.done.clone().to_string())
                ]));
        }

        // Imprimir la tabla
        table.printstd();
    }
}

