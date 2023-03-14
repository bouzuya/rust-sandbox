mod command;
mod config;

use adapter_fs::FsBRepository;
use clap_complete::{generate, Shell};
use config::{Config, ConfigRepository};
use entity::BId;
use limited_date_time::TimeZoneOffset;
use std::{io, path::PathBuf, str::FromStr};
use use_case::{HasBRepository, HasListUseCase, HasViewUseCase};

#[derive(Debug, clap::Parser)]
struct Opt {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    /// Prints the shell's completion script
    Completion {
        #[arg(name = "SHELL", help = "the shell", value_enum)]
        shell: Shell,
    },
    /// Lists b files
    List {
        #[arg(long)]
        json: bool,
        query: String,
    },
    /// Creates a new file
    New {
        /// The data file
        #[arg(short, long)]
        data_file: PathBuf,
        /// The template file or directory
        #[arg(short, long)]
        template: PathBuf,
    },
    /// Views the b file
    View {
        #[arg(name = "BID")]
        id: BId,
        /// View content data
        #[arg(long)]
        content: bool,
        /// View meta data
        #[arg(long)]
        meta: bool,
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
    let opt = <Opt as clap::Parser>::parse();
    match opt.subcommand {
        Subcommand::Completion { shell } => {
            let mut command = <Opt as clap::CommandFactory>::command();
            generate(shell, &mut command, "b", &mut io::stdout());
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
        Subcommand::View { content, id, meta } => {
            let config = ConfigRepository::new().load()?;
            let app = build_app(config)?;
            command::view(&app, content, id, meta, &mut io::stdout())
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
        command::view(&app, false, bid, false, &mut output).unwrap();
        assert_eq!(output, b"Hello, world!");
        Ok(())
    }
}
