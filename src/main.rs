fn main() {
    let word = std::env::args().nth(1).expect("send a name as argument");
    let seed: pixie::generator::Seed = word.into();
    let canva = pixie::generator::Canva::new(5, seed);
    pixie::rendering::Terminal
        .render(std::io::stdout(), canva.clone())
        .expect("failed to render image to terminal");

    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("pixie.png")
        .expect("failed to open file");

    pixie::rendering::Png
        .render(file, canva)
        .expect("failed to render image to terminal");
}
