use clap::Parser;

fn main() {
    let rst = pixie::cli::Cli::parse().execute();
    if let Some(error) = rst.err() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
