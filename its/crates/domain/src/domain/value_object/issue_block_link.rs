use crate::IssueId;

#[derive(Clone, Debug)]
pub struct IssueBlockLink(IssueId, IssueId);

impl IssueBlockLink {
    pub fn new(issue_id1: IssueId, issue_id2: IssueId) -> Self {
        Self(issue_id1, issue_id2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let issue_id1 = "1".parse()?;
        let issue_id2 = "2".parse()?;
        let _ = IssueBlockLink::new(issue_id1, issue_id2);
        Ok(())
    }
}
