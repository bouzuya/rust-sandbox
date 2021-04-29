use std::{env, fs};

fn main() {
    let file = env::args().nth(1).expect("The file is not specified.");
    let content = fs::read_to_string(&file).expect("Failed to load the file.");
    let rules = markdown_link_helper::build_rules();
    markdown_link_helper::run(&rules, &content);
}
