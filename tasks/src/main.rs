use structopt::StructOpt;
use tasks::{TaskJsonRepository, TaskRepository};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(about = "Adds a new task")]
    Add { text: String },
    #[structopt(about = "Completes the task")]
    Done { id: usize },
    #[structopt(about = "Lists tasks")]
    List,
    #[structopt(about = "Removes the task")]
    Remove { id: usize },
}

fn main() {
    let opt = Opt::from_args();
    let repository = TaskJsonRepository::new();
    match opt.subcommand {
        Subcommand::Add { text } => {
            repository.create(text);
        }
        Subcommand::Done { id } => {
            let mut task = repository.find_by_id(id).unwrap();
            task.done = true;
            repository.save(task);
        }
        Subcommand::List => {
            let tasks = repository.find_all();
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
        Subcommand::Remove { id } => {
            repository.delete(id);
        }
    }
}
