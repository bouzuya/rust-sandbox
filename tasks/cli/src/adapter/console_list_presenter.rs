use entity::Task;
use use_case::ListPresenter;

pub struct ConsoleListPresenter;

impl ConsoleListPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl ListPresenter for ConsoleListPresenter {
    fn complete(&self, tasks: &[Task]) {
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
