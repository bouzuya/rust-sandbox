use anyhow::Context;

use super::{entry::Entry, template::Template};
use std::{collections::BTreeMap, convert::TryFrom, fs, path::Path};

#[derive(Debug, Eq, PartialEq)]
pub enum TemplateEntry<'a> {
    TemplateDir {
        name: Template<'a>,
    },
    TemplateFile {
        name: Template<'a>,
        content: Template<'a>,
    },
}

impl<'a> TemplateEntry<'a> {
    pub fn render(&self, root_dir: &Path, data: &BTreeMap<String, String>) -> anyhow::Result<()> {
        match self {
            TemplateEntry::TemplateDir { name } => {
                let dest = root_dir.join(name.render(data)?);
                if !dest.exists() {
                    fs::create_dir(dest.as_path())?;
                }
                println!("{}", dest.as_path().to_str().context("to_str error")?);
                Ok(())
            }
            TemplateEntry::TemplateFile { name, content } => {
                let dest = root_dir.join(name.render(data)?);
                let content = content.render(data)?;
                fs::write(dest.as_path(), content)?;
                println!("{}", dest.as_path().to_str().context("to_str error")?);
                Ok(())
            }
        }
    }
}

impl<'a> TryFrom<&'a Entry> for TemplateEntry<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Entry) -> Result<Self, Self::Error> {
        match value {
            Entry::Dir { name } => {
                let name = Template::try_from(name.as_str())?;
                Ok(Self::TemplateDir { name })
            }
            Entry::File { content, name } => {
                let name = Template::try_from(name.as_str())?;
                let content = Template::try_from(content.as_str())?;
                Ok(Self::TemplateFile { name, content })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn render_test() {
        let dir = tempdir().unwrap();
        let entries = vec![
            Entry::Dir {
                name: "{{foo}}".to_string(),
            },
            Entry::File {
                name: "{{foo}}/{{bar}}".to_string(),
                content: "{{baz}}".to_string(),
            },
        ];
        let templates = entries
            .iter()
            .map(TemplateEntry::try_from)
            .collect::<Result<Vec<TemplateEntry>, _>>()
            .unwrap();
        let mut data = BTreeMap::new();
        data.insert("foo".to_string(), "FOO".to_string());
        data.insert("bar".to_string(), "BAR".to_string());
        data.insert("baz".to_string(), "BAZ".to_string());
        for t in templates {
            t.render(dir.path(), &data).unwrap();
        }

        assert!(dir.path().join("FOO").is_dir());
        assert!(dir.path().join("FOO").join("BAR").is_file());
        assert_eq!(
            fs::read_to_string(dir.path().join("FOO").join("BAR")).unwrap(),
            "BAZ".to_string()
        );
    }

    #[test]
    fn try_from_test() {
        assert_eq!(
            TemplateEntry::try_from(&Entry::Dir {
                name: "{{foo}}".to_string(),
            })
            .unwrap(),
            TemplateEntry::TemplateDir {
                name: Template::try_from("{{foo}}").unwrap()
            }
        );

        assert_eq!(
            TemplateEntry::try_from(&Entry::File {
                name: "{{foo}}/{{bar}}".to_string(),
                content: "{{baz}}".to_string(),
            })
            .unwrap(),
            TemplateEntry::TemplateFile {
                name: Template::try_from("{{foo}}/{{bar}}").unwrap(),
                content: Template::try_from("{{baz}}").unwrap()
            }
        );
    }
}
