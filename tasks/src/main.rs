use std::{env, fs};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TasksJson {
    tasks: Vec<String>,
}

fn main() {
    let data_dir = dirs::data_dir().unwrap();
    let path = data_dir.join("tasks.json");
    let json_string = if path.exists() {
        fs::read_to_string(path.as_path()).expect("read failed")
    } else {
        r#"{"tasks":[]}"#.to_owned()
    };
    let mut json: TasksJson = serde_json::from_str(json_string.as_str()).expect("invalid json");

    let command = env::args().nth(1).expect("no subcommand");
    if command.as_str() != "add" {
        panic!("invalid subcommand");
    }
    let text = env::args().nth(2).expect("no text");
    json.tasks.push(text);

    println!("{}", json.tasks.join("\n"));

    let json_string = serde_json::to_string(&json).expect("invalid data");
    fs::write(path.as_path(), json_string).expect("write failed");
}
