use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Task {
        id : u32,
        description : String,
        status : String,
    }

    fn add_task(tasks: &mut Vec<Task>, desc: &str, next_id: u32) -> u32 {
    let task = Task {
        id: next_id,
        description: desc.to_string(),
        status: "In Progress".to_string(),
    };
    tasks.push(task);
    next_id + 1
}
fn show_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("ID: {} | {} [{}]", task.id, task.description, task.status);
    }
}
fn mark_complete(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.status = "Done".to_string();
    } else {
        println!("Task not found.");
    }
}
fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    let len_before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() < len_before {
        println!("Task deleted.");
    } else {
        println!("Task not found.");
    }
}
fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create("tasks.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
fn load_tasks() -> io::Result<Vec<Task>> {
    let file = OpenOptions::new().read(true).open("tasks.json");

    if let Ok(file) = file {
        let reader = BufReader::new(file);
        let tasks = serde_json::from_reader(reader)?;
        Ok(tasks)
    } else {
        Ok(Vec::new())
    }
}


fn main() {
    let mut tasks = load_tasks().unwrap_or_else(|_| {
        println!("No existing task file found. Starting fresh.");
        Vec::new()
    });

    loop {
        println!("\n--- Task Manager ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Save and Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Flush to ensure the prompt shows

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut desc = String::new();
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut desc).unwrap();

                let id = tasks.len() as u32 + 1;
                let task = Task {
                    id,
                    description: desc.trim().to_string(),
                    status: "In Progress".to_string(),
                };
                tasks.push(task);
                println!("Task added.");
            }
            "2" => show_tasks(&tasks),
            "3" => {
                let mut id_input = String::new();
                print!("Enter task ID to complete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    mark_complete(&mut tasks, id);
                } else {
                    println!("Invalid ID.");
                }
            }
            "4" => {
                let mut id_input = String::new();
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    delete_task(&mut tasks, id);
                } else {
                    println!("Invalid ID.");
                }
            }
            "5" => {
                if let Err(e) = save_tasks(&tasks) {
                    println!("Failed to save tasks: {}", e);
                } else {
                    println!("Tasks saved. Goodbye!");
                }
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}


