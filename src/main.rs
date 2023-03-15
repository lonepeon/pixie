fn main() {
    if let Some(error) = run().err() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), pixie::error::Error> {
    let filename = "pixie.png";
    let size = 5;
    let word = std::env::args().nth(1).expect("send a name as argument");
    let seed: pixie::generator::Seed = word.into();
    let canva = pixie::generator::Canva::new(size, seed);

    pixie::rendering::Terminal.render(std::io::stdout(), canva.clone());

    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filename)
        .map_err(|e| {
            pixie::error::Error::from(e).context(format!("cannot open \"{}\"", filename))
        })?;

    pixie::rendering::Png
        .render(file, canva)
        .map_err(|e| e.context(format!("cannot generate PNG to \"{}\"", filename)))?;

    Ok(())
}
