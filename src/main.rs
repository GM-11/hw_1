#[derive(Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn add_task(description: &str, tasks: &mut Vec<Task>) -> Task {
    let task = Task {
        id: tasks.clone().len(),
        description: String::from(description),
        completed: false,
    };

    tasks.push(task.clone());

    task
}

fn complete_task(id: usize, tasks: &mut Vec<Task>) -> Option<&Task> {
    let selected_task = tasks.iter_mut().find(|t| t.id == id);
    // selected_task
    match selected_task {
        Some(task) => task.completed = true,
        None => (),
    }
    tasks.iter().find(|t| t.id == id)
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!(
            "ID: {} | Description: {} | Completed: {}",
            task.id, task.description, task.completed
        );
    }
}

fn main() {
    println!("Tasks");

    let mut tasks: Vec<Task> = Vec::new();

    add_task("Complete homework", &mut tasks);
    add_task("Play guitar", &mut tasks);
    add_task("Watch movie", &mut tasks);
    add_task("Go to gym", &mut tasks);

    complete_task(1 as usize, &mut tasks);
    complete_task(2 as usize, &mut tasks);
    complete_task(3 as usize, &mut tasks);

    list_tasks(&tasks);
}
