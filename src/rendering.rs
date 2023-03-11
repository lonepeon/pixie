pub trait Renderer {
    fn render<W: std::io::Write>(
        &mut self,
        w: W,
        canva: crate::generator::Canva,
    ) -> Result<(), String>;
}

pub struct Terminal;

impl Renderer for Terminal {
    fn render<W: std::io::Write>(
        &mut self,
        mut w: W,
        canva: crate::generator::Canva,
    ) -> Result<(), String> {
        let mut current_line = 0;
        canva.into_iter().for_each(|(pt, shown)| {
            if pt.y > current_line {
                current_line = pt.y;
                writeln!(w).expect("failed to write newline");
            }

            let pattern = if shown { "█" } else { " " };
            write!(w, "{}", pattern).expect("failed to write character");
        });
        writeln!(w).expect("failed to write newline");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Renderer;

    #[test]
    fn terminal_render() {
        let canva = crate::generator::Canva::new(5, "hello".into());
        let mut buffer = Vec::new();

        super::Terminal
            .render(&mut buffer, canva)
            .expect("failed to render");

        let output = String::from_utf8(buffer).expect("failed to cast bytes to string");

        let expect = include_str!("../testdata/terminal_render.ascii");
        assert_eq!(expect, output)
    }
}
