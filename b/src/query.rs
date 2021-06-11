use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Query {
    pub date: String,
    pub tags: Option<Vec<String>>,
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.date,
            match self.tags {
                Some(ref tags) => format!("{}{}", " tag:", tags.join(" tag:")),
                None => "".to_string(),
            }
        )
    }
}

impl FromStr for Query {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split(' ').collect::<Vec<&str>>();
        Ok(Query {
            date: ss[0].to_string(),
            tags: if ss[1..].is_empty() {
                None
            } else {
                Some(
                    ss[1..]
                        .iter()
                        .map(|s| s.trim_start_matches("tag:").to_string())
                        .collect::<Vec<String>>(),
                )
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_convert_test() {
        assert_eq!(
            Query::from_str("2021-02-03").unwrap(),
            Query {
                date: "2021-02-03".to_string(),
                tags: None
            }
        );
        assert_eq!(
            Query::from_str("2021-02-03 tag:tag1").unwrap(),
            Query {
                date: "2021-02-03".to_string(),
                tags: Some(vec!["tag1".to_string()])
            }
        );
        assert_eq!(
            Query::from_str("2021-02-03 tag:t1 tag:t2")
                .unwrap()
                .to_string(),
            "2021-02-03 tag:t1 tag:t2".to_string()
        );
    }
}
