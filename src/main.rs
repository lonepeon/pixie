use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliOutput {
    Terminal,
    Png,
}

impl std::fmt::Display for CliOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Terminal => "term",
            Self::Png => "png",
        };

        write!(f, "{}", value)
    }
}

impl std::str::FromStr for CliOutput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "term" => Ok(Self::Terminal),
            "png" => Ok(Self::Png),
            value => Err(format!("unsupported output format '{}'", value)),
        }
    }
}

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short='o', long="output", default_value_t = CliOutput::Terminal, help = "format of the generated image (term=ascii characters, png=png file)")]
    output: CliOutput,
    #[arg(
        short = 's',
        long = "size",
        default_value_t = 10,
        help = "size of the pixel grid"
    )]
    size: usize,
    #[arg(
        short = 'f',
        long = "file",
        default_value = "pixie.png",
        help = "file where the image should be written. Only used by the PNG output."
    )]
    filename: String,

    #[arg(help = "word used as a base value to generate the image")]
    word: String,
}

fn main() {
    if let Some(error) = run().err() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), pixie::error::Error> {
    let cli = Cli::parse();

    let seed: pixie::generator::Seed = cli.word.into();
    let canva = pixie::generator::Canva::new(cli.size, seed);

    match cli.output {
        CliOutput::Terminal => pixie::rendering::Terminal.render(std::io::stdout(), canva),
        CliOutput::Png => {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&cli.filename)
                .map_err(|e| {
                    pixie::error::Error::from(e)
                        .context(format!("cannot open \"{}\"", cli.filename))
                })?;
            pixie::rendering::Png
                .render(file, canva)
                .map_err(|e| e.context(format!("cannot generate PNG to \"{}\"", cli.filename)))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn cli_output_from_string_term() {
        let out = super::CliOutput::from_str("term").expect("failed to build a valid CLI output");

        assert_eq!(super::CliOutput::Terminal, out)
    }

    #[test]
    fn cli_output_from_string_png() {
        let out = super::CliOutput::from_str("png").expect("failed to build a valid CLI output");

        assert_eq!(super::CliOutput::Png, out)
    }

    #[test]
    fn cli_output_from_string_unexpected() {
        let err = super::CliOutput::from_str("nope").err();

        assert_eq!(Some("unsupported output format 'nope'".to_string()), err)
    }

    #[test]
    fn cli_output_display_term() {
        assert_eq!("term", format!("{}", super::CliOutput::Terminal))
    }

    #[test]
    fn cli_output_display_png() {
        assert_eq!("png", format!("{}", super::CliOutput::Png))
    }
}
