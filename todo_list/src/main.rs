use dialoguer::{Select, Input};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    done: bool,
}

fn setup_database() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (name, done) VALUES (?1, ?2)",
        params![task.name, task.done],
    )?;
    Ok(())
}

fn remove_task(conn: &Connection, task_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        params![task_id],
    )?;
    Ok(())
}

fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, name, done FROM tasks")?; // Retrieve ID
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

fn get_names(tasks: &[Task]) -> Vec<String> {
    tasks.iter().map(|task| task.name.clone()).collect()
}

fn main() -> Result<()> {
    println!("Welcome to TODOLIST");
    let conn = setup_database()?;
    
    loop {
        let tasks = get_tasks(&conn)?;
        let mut names = get_names(&tasks);
        names.push(String::from("Add a task"));
        names.push(String::from("Return"));
        
        let selection = Select::new()
            .with_prompt("What do you choose?")
            .items(&names)
            .interact()
            .unwrap();
        
        println!("You chose: {}", names[selection]);
        
        if names[selection] == "Return" {
            break;
        } else if names[selection] == "Add a task" {
            let new_task_name: String = Input::new()
                .with_prompt("Name your new task:")
                .interact_text()
                .unwrap();
            
            let new_task = Task {
                id: 0,
                name: new_task_name,
                done: false,
            };
            insert_task(&conn, &new_task)?;
        } else {
            let task_to_remove = &tasks[selection];
            remove_task(&conn, task_to_remove.id)?;
            println!("Removed task: {:?}", task_to_remove);
        }
    }
    
    Ok(())
}
