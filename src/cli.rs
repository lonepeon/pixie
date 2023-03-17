use std::os::fd::FromRawFd;

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
pub struct Cli {
    #[arg(
        short='o',
        long="output",
        default_value_t = CliOutput::Terminal,
        help = "format of the generated image (term=ascii characters, png=png file)"
    )]
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
        default_value = "-",
        help = "file where the image should be written. '-' is used to mean stdout."
    )]
    filename: String,

    #[arg(help = "word used as a base value to generate the image")]
    word: String,
}

impl Cli {
    fn file(&self) -> Result<std::fs::File, crate::error::Error> {
        if self.filename == "-" {
            unsafe {
                // this is a hack in order to make stdout seekable. It is required by
                // the PNG renderer
                //
                // source: https://github.com/rust-lang/rust/issues/72802#issuecomment-1101996578
                use std::os::unix::io::AsRawFd;
                Ok(std::fs::File::from_raw_fd(std::io::stdout().as_raw_fd()))
            }
        } else {
            std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&self.filename)
                .map_err(|e| {
                    crate::error::Error::from(e)
                        .context(format!("cannot open \"{}\"", self.filename))
                })
        }
    }

    pub fn execute(&self) -> Result<(), crate::error::Error> {
        let seed: crate::generator::Seed = self.word.as_str().into();
        let canva = crate::generator::Canva::new(self.size, seed);
        let file = self.file()?;

        match self.output {
            CliOutput::Terminal => crate::rendering::Terminal.render(file, canva),
            CliOutput::Png => {
                crate::rendering::Png.render(file, canva).map_err(|e| {
                    e.context(format!("cannot generate PNG to \"{}\"", self.filename))
                })?;
            }
        }

        Ok(())
    }
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

    #[test]
    fn cli_execute_ascii() {
        let file = Tempfile::new();

        let cli = super::Cli {
            output: super::CliOutput::Terminal,
            word: "hello".to_string(),
            size: 5,
            filename: file.name.to_string(),
        };

        cli.execute().expect("failed to generate image");

        let expect = include_str!("../testdata/terminal_render.ascii");
        let actual = std::fs::read_to_string(&file.name).expect("failed to read generated image");

        assert_eq!(expect, actual)
    }

    #[test]
    fn cli_execute_png() {
        let file = Tempfile::new();

        let cli = super::Cli {
            output: super::CliOutput::Png,
            word: "hello".to_string(),
            size: 5,
            filename: file.name.to_string(),
        };

        cli.execute().expect("failed to generate image");

        let expect =
            std::fs::read("testdata/png_render.png").expect("failed to read expected image");
        let actual = std::fs::read(&file.name).expect("failed to read generated image");

        assert_eq!(expect, actual)
    }

    struct Tempfile {
        name: String,
    }

    impl Tempfile {
        fn new() -> Self {
            let id = uuid::Uuid::new_v4();
            let name = format!("{}/{}", std::env::temp_dir().display(), id);
            Tempfile { name }
        }
    }

    impl Drop for Tempfile {
        fn drop(&mut self) {
            std::fs::remove_file(&self.name).expect("failed to remove temp file");
        }
    }
}
