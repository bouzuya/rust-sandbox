#[derive(Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl std::str::FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        anyhow::ensure!(parts.len() == 2);
        let x = i64::from_str(parts[0])?;
        let y = i64::from_str(parts[1])?;
        Ok(Self { x, y })
    }
}

#[derive(clap::Parser)]
struct Args {
    /// The path to the input PDF file
    input: std::path::PathBuf,
    /// The path to the stamp image file
    stamp: std::path::PathBuf,
    /// The path to the output PDF file
    #[arg(long)]
    output: Option<std::path::PathBuf>,
    /// The position of the stamp image as x,y (top-left is 0,0)
    #[arg(long)]
    position: Option<Position>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    println!(
        "pdfoin{}{} {} {}",
        args.output
            .as_ref()
            .map(|it| format!(" --output {}", it.display()))
            .unwrap_or_default(),
        args.position
            .as_ref()
            .map(|it| format!(" --position {},{}", it.x, it.y))
            .unwrap_or_default(),
        args.input.display(),
        args.stamp.display()
    );
}
