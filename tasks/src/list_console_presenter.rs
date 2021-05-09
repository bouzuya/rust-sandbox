use tasks::{use_case::ListPresenter, Task};

pub struct ListConsolePresenter;

impl ListConsolePresenter {
    pub fn new() -> Self {
        Self
    }
}

impl ListPresenter for ListConsolePresenter {
    fn complete(&self, tasks: &Vec<Task>) {
        println!(
            "{}",
            tasks
                .iter()
                .map(|task| format!(
                    "{} {} {}",
                    task.id,
                    if task.done { "☑" } else { "☐" },
                    task.text
                ))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
