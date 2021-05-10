use serde_json::Value;
use std::{collections::BTreeMap, fs, path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "new", about = "Creates a new file")]
    New {
        #[structopt(short = "d", long = "data-file", help = "The data file")]
        data_file: String,
        #[structopt(short = "t", long = "template-file", help = "The template file")]
        template_file: String,
    },
}

fn render(template: &str, data: &BTreeMap<String, String>) -> String {
    let mut t = String::new();
    let mut expr: Option<String> = None;
    for c in template.chars() {
        match expr {
            Some(mut s) => match c {
                '{' => panic!(),
                '}' => {
                    let v = data.get(&s).unwrap();
                    t.push_str(v.as_str());
                    expr = None;
                }
                c => {
                    if !c.is_ascii_lowercase() {
                        panic!();
                    }
                    s.push(c);
                    expr = Some(s);
                }
            },
            None => match c {
                '{' => expr = Some(String::new()),
                c => t.push(c),
            },
        }
    }
    t
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::New {
            data_file,
            template_file,
        } => {
            let data = {
                let data = fs::read_to_string(&data_file).unwrap();
                let data: Value = serde_json::from_str(&data).unwrap();
                let object = data.as_object().unwrap();
                let mut map = BTreeMap::new();
                for (k, v) in object {
                    if !k.chars().all(|c| c.is_ascii_lowercase()) {
                        panic!();
                    }
                    let v = v.as_str().unwrap().to_string();
                    map.insert(k.clone(), v);
                }
                map
            };

            let template = fs::read_to_string(&template_file).unwrap();

            let template_path = PathBuf::from_str(template_file.as_str()).unwrap();
            let dest_file = template_path.file_name().unwrap().to_str().unwrap();
            let dest_file = render(dest_file, &data);
            let content = render(&template, &data);

            fs::write(dest_file.as_str(), content).unwrap();
            println!("{}", dest_file.as_str());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), "y".to_string());
        assert_eq!(render("{x}", &map), "y".to_string());
    }
}
