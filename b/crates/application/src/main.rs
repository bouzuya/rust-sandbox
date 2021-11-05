mod command;
mod config;

use adapter_fs::FsBRepository;
use config::{Config, ConfigRepository};
use entity::BId;
use limited_date_time::TimeZoneOffset;
use std::{io, path::PathBuf, str::FromStr};
use structopt::{clap::Shell, StructOpt};
use use_case::{HasBRepository, HasListUseCase, HasViewUseCase};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[structopt(name = "SHELL", help = "the shell", possible_values = &Shell::variants())]
        shell: Shell,
    },
    #[structopt(name = "list", about = "Lists b files")]
    List {
        #[structopt(long = "json")]
        json: bool,
        query: String,
    },
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
    #[structopt(name = "view", about = "Views the b file")]
    View {
        #[structopt(name = "BID")]
        id: BId,
    },
}

// FIXME:
struct App {
    brepository: FsBRepository,
}

impl HasBRepository for App {
    type BRepository = FsBRepository;

    fn b_repository(&self) -> &Self::BRepository {
        &self.brepository
    }
}

impl HasListUseCase for App {
    type ListUseCase = App;

    fn list_use_case(&self) -> &Self::ListUseCase {
        self
    }
}

impl HasViewUseCase for App {
    type ViewUseCase = App;

    fn view_use_case(&self) -> &Self::ViewUseCase {
        self
    }
}

fn build_app(config: Config) -> anyhow::Result<App> {
    let data_dir = config.data_dir();
    let time_zone_offset = TimeZoneOffset::from_str(config.time_zone_offset())?;
    let brepository = FsBRepository::new(data_dir.to_path_buf(), time_zone_offset);
    let app = App { brepository };
    Ok(app)
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("b", shell, &mut io::stdout());
            Ok(())
        }
        Subcommand::List { json, query } => {
            let config = ConfigRepository::new().load()?;
            let app = build_app(config)?;
            command::list(&app, json, query, &mut io::stdout())
        }
        Subcommand::New {
            data_file,
            template,
        } => {
            // TODO: use App
            command::new(data_file, template)
        }
        Subcommand::View { id } => {
            let config = ConfigRepository::new().load()?;
            let app = build_app(config)?;
            command::view(&app, id, &mut io::stdout())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::use_case::{BRepository, ListUseCase, Query};
    use adapter_fs::FsBRepository;
    use std::{fs, str::FromStr};
    use tempfile::tempdir;

    #[test]
    fn list_test() {
        let dir = tempdir().unwrap();
        let dir20210202 = dir.path().join("flow").join("2021").join("02").join("02");
        let dir20210203 = dir.path().join("flow").join("2021").join("02").join("03");
        fs::create_dir_all(dir20210202.as_path()).unwrap();
        fs::create_dir_all(dir20210203.as_path()).unwrap();
        let files = vec![
            dir20210202.join("20210202T145959Z.md"),
            dir20210202.join("20210202T150000Z.md"),
            dir20210202.join("20210202T235959Z.md"),
            dir20210203.join("20210203T000000Z.md"),
            dir20210203.join("20210203T145959Z.md"),
            dir20210203.join("20210203T150000Z.md"),
        ];
        for (i, f) in files.iter().enumerate() {
            fs::write(f.as_path(), format!("{}", i)).unwrap();
            fs::write(f.as_path().with_extension("json"), "{}").unwrap();
        }
        let query = Query::from_str("2021-02-03").unwrap();
        let repository = FsBRepository::new(
            dir.path().to_path_buf(),
            TimeZoneOffset::from_str("+09:00").unwrap(),
        );
        let app = App {
            brepository: repository,
        };
        let use_case = app.list_use_case();
        assert_eq!(
            use_case
                .handle(&query)
                .unwrap()
                .into_iter()
                .map(|p| app.b_repository().to_content_path_buf(&p.id))
                .collect::<Vec<PathBuf>>(),
            files[1..1 + 4]
        );
        assert_eq!(
            use_case
                .handle(&query)
                .unwrap()
                .into_iter()
                .map(|p| p.title)
                .collect::<Vec<String>>(),
            vec!["1", "2", "3", "4"]
        );
    }

    #[test]
    fn view_test() -> anyhow::Result<()> {
        let dir = tempdir().unwrap();
        let dir20210203 = dir.path().join("flow").join("2021").join("02").join("03");
        fs::create_dir_all(dir20210203.as_path()).unwrap();
        let meta = dir20210203.join("20210203T000000Z.json");
        fs::write(meta.as_path(), "{}").unwrap();
        let content = meta.with_extension("md");
        fs::write(content.as_path(), "Hello, world!").unwrap();
        let bid = BId::from_str("20210203T000000Z")?;
        let mut output = vec![];
        let repository = FsBRepository::new(
            dir.path().to_path_buf(),
            TimeZoneOffset::from_str("+09:00").unwrap(),
        );
        let app = App {
            brepository: repository,
        };
        command::view(&app, bid, &mut output).unwrap();
        assert_eq!(output, b"Hello, world!");
        Ok(())
    }
}
