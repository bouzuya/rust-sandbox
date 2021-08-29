use entity::Task;

pub struct ConsolePresenter;

impl ConsolePresenter {
    pub fn new() -> Self {
        Self
    }

    pub fn complete(&self, tasks: &[Task]) {
        println!(
            "{}",
            tasks
                .iter()
                .map(|task| format!(
                    "{} {} {}",
                    usize::from(task.id()),
                    if task.done() { "☑" } else { "☐" },
                    task.text()
                ))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
