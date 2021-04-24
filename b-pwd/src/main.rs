use std::env;

fn main() {
    println!(
        "{}",
        env::current_dir()
            .expect("current_dir failed")
            .to_str()
            .expect("current_dir is not UTF-8")
    );
}
