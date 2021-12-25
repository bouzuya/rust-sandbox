#[derive(Debug)]
struct Issue {}

impl Issue {
    pub fn new() -> Self {
        Self {}
    }
}

fn main() {
    let issue = Issue::new();
    println!("issue created : {:?}", issue);
}
