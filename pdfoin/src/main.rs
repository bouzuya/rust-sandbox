use anyhow::Context;

#[derive(Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
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
    #[arg(long, default_value = "output.pdf")]
    output: Option<std::path::PathBuf>,
    /// The position of the stamp image as x,y (top-left is 0,0)
    #[arg(long, default_value = "0,0")]
    position: Option<Position>,
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    let input = args.input;
    let output = args.output.context("output is none")?;
    let position = args.position.context("position is none")?;
    let stamp = args.stamp;

    // println!(
    //     "pdfoin --output {} --position {} {} {}",
    //     output.display(),
    //     position,
    //     input.display(),
    //     stamp.display()
    // );

    let mut document = ::lopdf::Document::load(input).context("load input pdf")?;

    // FIXME: load image file
    // FIXME: insert image to pdf

    let file = std::fs::File::create_new(&output).context("create output pdf")?;
    let mut writer = std::io::BufWriter::new(file);
    document.save_to(&mut writer).context("write output pdf")?;
    println!("The PDF file is output to {}", output.display());

    Ok(())
}
