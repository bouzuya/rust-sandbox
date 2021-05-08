use structopt::StructOpt;
use tasks::{TaskJsonRepository, TaskRepository};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
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
    match opt.sub_command {
        SubCommand::Add { text } => {
            repository.create(text);
        }
        SubCommand::Done { id } => {
            let mut task = repository.find_by_id(id).unwrap();
            task.done = true;
            repository.save(task);
        }
        SubCommand::List => {
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
        SubCommand::Remove { id } => {
            repository.delete(id);
        }
    }
}
