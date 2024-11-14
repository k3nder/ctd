use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use clap::{Parser, Subcommand};
use kommons_macros::{read_file, read_input};
use crate::objects::{Project, ToDo, ToDoFile, TODO_FILE_LOC};

mod objects;

#[derive(Debug, Parser)]
#[command(author = "k3nder", version = "1.0", about = "A simple todo manager")]
struct Args {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Debug, Subcommand)]
enum Actions {
    create {
        #[command(subcommand)]
        typ: RegisterType
    },
    remove {
        #[command(subcommand)]
        typ: RegisterType
    },
    done {
        #[command(subcommand)]
        typ: RegisterType,
    },
    edit {
        #[command(subcommand)]
        typ: RegisterType,
    },
    view {}
}

#[derive(Debug, Subcommand)]
enum RegisterType {
    task {
        #[arg()]
        name: String,
        #[arg(short = 'd', long = "description")]
        description: Option<String>,
        #[arg(short = 'p', long = "priority")]
        priority: Option<i32>,
        #[arg(short = 'l', long = "limit")]
        limit: Option<String>
    },
    project {
        #[arg()]
        name: String,
        #[arg(short = 'd', long = "description")]
        description: Option<String>,
    }
}

fn main() {
    if !Path::new(&TODO_FILE_LOC.clone()).exists() {
        let todo_file = ToDoFile {
            to_do: Default::default(),
            projects: Default::default(),
        };

        todo_file.save();
    }
    let args = Args::parse();
    execute(args);
    //let todo = ToDoFile::deserialize(&read_file!(TODO_FILE_LOC));
    //todo.unwrap().print_tasks();
}

fn execute(args: Args) {
    match args.action {
        Actions::create { typ } => execute_create(typ),
        Actions::remove { typ } => execute_remove(typ),
        Actions::done { typ } => execute_done(typ),
        Actions::view {} => execute_view(),
        Actions::edit { typ } => execute_create(typ),
        _ => {}
    }
}
fn execute_view() {
    let todo = ToDoFile::deserialize(&read_file!(TODO_FILE_LOC.clone())).expect("Error deserializing todo");
    todo.print_tasks();
}
fn execute_done(typ: RegisterType) {
    let mut todo_file = ToDoFile::deserialize(&read_file!(TODO_FILE_LOC.clone()))
        .expect("Error deserializing todo file");
    match typ {
        RegisterType::task { name, description, priority, limit } => {

            if name.clone().contains("/") {
                let name = name.split("/").collect::<Vec<&str>>();
                if todo_file.projects.contains_key(name[0]) { eprintln!("Project {} not found", name[0]); exit(1); }
                let mut project = todo_file.projects.get_mut(name[0]).unwrap();

                if !project.to_do.contains_key(name[1]) {
                    eprintln!("Project task {} no exist", name[1]);
                    exit(1);
                }

                project.to_do.get_mut(name[1]).unwrap().done = true;
                return;
            }

            if !todo_file.to_do.contains_key(&name) {
                eprintln!("Task {} no exist", name);
                exit(1);
            }

            todo_file.to_do.get_mut(&name).unwrap().done = true;

            println!("¡DONE!");
        },
        _ => {}
    }

    todo_file.save();
}
fn execute_remove(typ: RegisterType) {
    let mut todo_file = ToDoFile::deserialize(&read_file!(TODO_FILE_LOC.clone()))
        .expect("Error deserializing todo file");
    match typ {
        RegisterType::task { name, description, priority, limit } => {

            println!("Deleting task..");

            if name.clone().contains("/") {
                let name = name.split("/").collect::<Vec<&str>>();
                if todo_file.projects.contains_key(name[0]) { eprintln!("Project {} not found", name[0]); exit(1); }
                let mut project = todo_file.projects.get_mut(name[0]).unwrap();

                if !project.to_do.contains_key(name[1]) {
                    eprintln!("Project task {} no exist", name[1]);
                    exit(1);
                }

                project.to_do.remove(name[1]);
                return;
            }

            if !todo_file.to_do.contains_key(&name) {
                eprintln!("Task {} no exist", name);
                exit(1);
            }

            todo_file.to_do.remove(&name);

            println!("Task deleted task");
        },
        RegisterType::project { name, description } => {
            if !todo_file.projects.contains_key(&name) {
                eprintln!("Project {} no exist", name);
                exit(1);
            }

            println!("Deleting project...");

            todo_file.projects.remove(&name);
            println!("Project deleted");
        }
    }

    todo_file.save();
}

fn execute_create(typ: RegisterType) {
    let mut todo_file = ToDoFile::deserialize(&read_file!(TODO_FILE_LOC.clone()))
        .expect("Error deserializing todo file");
    match typ {
        RegisterType::task { name, description, priority, limit } => {
            let description = if let None = description {
                read_input!("Description? ")
            } else { description.unwrap() };

            let priority = if let None = priority {
                read_input!("Priority? ").parse::<i32>().unwrap_or(0)
            } else { priority.unwrap() };

            let limit = if let None = limit {
                let limit = read_input!("Limit (enter for none)? ");
                if limit.is_empty() { None } else { Some(limit) }
            } else { Some(limit.unwrap()) };

            println!("Creating task");

            if name.clone().contains("/") {
                let name = name.split("/").collect::<Vec<&str>>();
                if todo_file.projects.contains_key(name[0]) { eprintln!("Project {} not found", name[0]); exit(1); }
                let mut project = todo_file.projects.get_mut(name[0]).unwrap();

                if project.to_do.contains_key(name[1]) {
                    eprintln!("Project task {} already exists", name[1]);
                    exit(1);
                }

                project.to_do.insert(name[1].to_string(), ToDo {
                    title: name[1].to_string(),
                    description,
                    priority,
                    limit,
                    done: false
                });
                return;
            }

            if todo_file.to_do.contains_key(&name) {
                eprintln!("Task {} already exists", name);
                exit(1);
            }

            todo_file.to_do.insert(name.clone(), ToDo {
                title: name,
                description,
                priority,
                limit,
                done: false,
            });

            println!("Created task");
        },
        RegisterType::project { name, description } => {
            if todo_file.projects.contains_key(&name) {
                eprintln!("Project {} already exists", name);
                exit(1);
            }

            let description = if let None = description {
                read_input!("Description? ")
            } else { description.unwrap() };

            println!("Creating project");

            todo_file.projects.insert(name.clone() ,Project {
                title: name,
                description,
                to_do: HashMap::new(),
            });
            println!("Created project");
        }
    }

    todo_file.save();
}