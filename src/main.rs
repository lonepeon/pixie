use pixie::rendering::Renderer;

fn main() {
    let seed = "plop".into();
    let canva = pixie::generator::Canva::new(5, seed);
    pixie::rendering::Terminal
        .render(std::io::stdout(), canva)
        .expect("failed to render image to terminal");
}
