use ulid::Ulid;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct IssueId(Ulid);

impl IssueId {
    pub fn new(id: Ulid) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
struct Issue {
    id: IssueId,
}

impl Issue {
    pub fn new(id: IssueId) -> Self {
        Self { id }
    }
}

fn main() {
    let issue = Issue::new(IssueId::new(Ulid::new()));
    println!("issue created : {:?}", issue);
}
