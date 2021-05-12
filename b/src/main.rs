use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};
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
        data_file: PathBuf,
        #[structopt(
            short = "t",
            long = "template",
            help = "The template file or directory"
        )]
        template: PathBuf,
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

fn read_dir(dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = vec![];
    for entry in dir.read_dir()? {
        entries.push(entry?);
    }
    Ok(entries)
}

fn list(template_file: &Path) -> Vec<String> {
    let mut file_names = vec![];
    file_names.push(template_file.to_str().unwrap().to_string());
    if template_file.is_dir() {
        list2(&mut file_names, template_file);
    }
    file_names
}

fn list2(file_names: &mut Vec<String>, dir: &Path) {
    let entries = read_dir(dir).expect("read_dir failed");
    for entry in entries {
        let path_buf = entry.path();
        let file_name = path_buf
            .to_str()
            .expect("file_name is not string")
            .to_string();
        let file_type = entry.file_type().expect("file_type failed");
        let is_dir = file_type.is_dir();
        file_names.push(file_name);
        if is_dir {
            list2(file_names, path_buf.as_path());
        }
    }
}

fn create(tmpl: &Path, root_dir: &Path, data: &BTreeMap<String, String>) {
    let file_name_tmpl = tmpl.strip_prefix(root_dir).unwrap().to_str().unwrap();
    let dest = render(file_name_tmpl, &data);
    if tmpl.is_dir() {
        if !PathBuf::from_str(dest.as_str()).unwrap().exists() {
            fs::create_dir(dest.as_str()).unwrap();
        }
    } else {
        let content_tmpl = fs::read_to_string(tmpl).unwrap();
        let content = render(content_tmpl.as_str(), &data);
        fs::write(dest.as_str(), content).unwrap();
    }
    println!("{}", dest.as_str());
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::New {
            data_file,
            template,
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

            let file_names = list(template.as_path());
            let root_dir = template.parent().unwrap();
            for file_name in file_names {
                create(PathBuf::from(file_name).as_path(), root_dir, &data);
            }
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
