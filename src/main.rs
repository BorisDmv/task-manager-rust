use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    deadline: String,
    priority: String,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "BorisDmv", version = "1.0.0", about = "Create tasks and give them deadline and priority and then complete them", long_about = None)]
struct Args {
    /// Name of the task
    #[arg(short, long)]
    name: Option<String>,

    #[arg(long, default_value = "None")]
    deadline: String,

    #[arg(long, default_value = "Medium")]
    priority: String,

    /// Number of times to create the task
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Print all tasks
    #[arg(long)]
    all: bool,

    /// Give the task index to be removed from the list
    #[arg(long)]
    remove: Option<u32>,

    /// Write the "tasks.json" so it will be deleted
    #[arg(short, long)]
    delete: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(file_to_delete) = args.delete {
        // Delete the specified file if the `delete` argument is provided
        delete_json_file(&file_to_delete);
    } else if args.all {
        // Print all items from the JSON file
        print_all_items_from_json();
    } else if let Some(index_to_remove) = args.remove {
        // Remove a specific task by index or ID
        remove_task_by_index(index_to_remove);
    } else {
        // Read the existing tasks from the JSON file
        let file_name = "tasks.json";
        let existing_tasks = read_existing_tasks_from_file(file_name);

        // Create new tasks
        let mut new_tasks: Vec<Task> = Vec::new();

        for _ in 0..args.count {
            let task = Task {
                name: args.name.clone().unwrap_or_default(),
                deadline: args.deadline.clone(),
                priority: args.priority.clone(),
            };
            new_tasks.push(task);
        }

        // Combine the new tasks with the existing tasks
        let mut combined_tasks = existing_tasks;
        combined_tasks.extend(new_tasks);

        // Serialize combined tasks to JSON
        let json_tasks = serde_json::to_string_pretty(&combined_tasks).unwrap();

        // Write JSON data to the file
        let mut file = File::create(file_name).expect("Failed to create the JSON file");
        file.write_all(json_tasks.as_bytes())
            .expect("Failed to write to the JSON file");

        println!("Tasks saved to {}.", file_name);
    }
}

fn read_existing_tasks_from_file(file_name: &str) -> Vec<Task> {
    match std::fs::read_to_string(file_name) {
        Ok(contents) => serde_json::from_str(&contents).unwrap(),
        Err(err) => {
            eprintln!("Error reading the JSON file: {}", err);
            Vec::new() // Return an empty vector if there was an error
        }
    }
}

fn print_all_items_from_json() {
    let file_name = "tasks.json";
    match std::fs::read_to_string(file_name) {
        Ok(contents) => {
            let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap();
            for (index, task) in tasks.iter().enumerate() {
                println!(
                    "Task {}:",
                    format!("Task {} with index {}", index + 1, index).yellow()
                );
                println!("Name: {}", task.name.green());
                println!("Deadline: {}", task.deadline.red());
                println!("Priority: {}", task.priority.blue());
                println!();
            }
        }
        Err(err) => {
            eprintln!("Error reading the JSON file: {}", err);
        }
    }
}

fn remove_task_by_index(index: u32) {
    let file_name = "tasks.json";
    let mut tasks: Vec<Task> = match std::fs::read_to_string(file_name) {
        Ok(contents) => serde_json::from_str(&contents).unwrap(),
        Err(err) => {
            eprintln!("Error reading the JSON file: {}", err);
            return;
        }
    };

    // Check if the provided index is within bounds
    if (index as usize) < tasks.len() {
        tasks.remove(index as usize);

        // Serialize tasks back to JSON and save to the file
        let json_tasks = serde_json::to_string_pretty(&tasks).unwrap();
        let mut file = File::create(file_name).expect("Failed to create the JSON file");
        file.write_all(json_tasks.as_bytes())
            .expect("Failed to write to the JSON file");

        println!("Task at index {} removed.", index);
    } else {
        eprintln!("Task index {} is out of bounds.", index);
    }
}

fn delete_json_file(file_name: &str) {
    if Path::new(file_name).exists() {
        if let Err(err) = fs::remove_file(file_name) {
            eprintln!("Error deleting the file: {}", err);
        } else {
            println!("File {} deleted.", file_name);
        }
    } else {
        eprintln!("File {} does not exist.", file_name);
    }
}
