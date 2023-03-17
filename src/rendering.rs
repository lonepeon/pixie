pub struct Terminal;

impl Terminal {
    fn ansi_color(color: crate::generator::Color) -> usize {
        match color {
            crate::generator::Color::Red => 160,
            crate::generator::Color::Blue => 33,
            crate::generator::Color::Pink => 199,
            crate::generator::Color::Green => 40,
            crate::generator::Color::Brown => 130,
            crate::generator::Color::Purple => 140,
            crate::generator::Color::Yellow => 226,
            crate::generator::Color::Black => 232,
        }
    }

    pub fn render<W: std::io::Write>(&mut self, mut w: W, canva: crate::generator::Canva) {
        let line = "──".repeat(canva.size());
        let color = Self::ansi_color(canva.color());

        writeln!(w, "\x1b[38;5;{};48;5;15m┌─{}─┐\x1b[0m", color, line)
            .expect("failed to write top border");
        write!(w, "\x1b[38;5;{};48;5;15m│ ", color).expect("failed to write beginning of drawing");

        let mut current_line = 0;
        canva.into_iter().for_each(|(pt, shown)| {
            if pt.y > current_line {
                current_line = pt.y;
                writeln!(w, " │\x1b[0m").expect("failed to write end of line");
                write!(w, "\x1b[38;5;{};48;5;15m│ ", color).expect("failed to write end of line");
            }

            let pattern = if shown { "██" } else { "  " };
            write!(w, "{}", pattern).expect("failed to write character");
        });
        writeln!(w, " │\x1b[0m").expect("failed to write end of drawing");
        writeln!(w, "\x1b[38;5;{};48;5;15m└─{}─┘\x1b[0m", color, line)
            .expect("failed to write bottom border");
    }
}

pub struct Png;

impl Png {
    fn rgb_color(color: crate::generator::Color) -> image::Rgb<u8> {
        match color {
            crate::generator::Color::Red => image::Rgb([222, 48, 48]),
            crate::generator::Color::Blue => image::Rgb([48, 146, 227]),
            crate::generator::Color::Pink => image::Rgb([227, 97, 177]),
            crate::generator::Color::Green => image::Rgb([109, 212, 123]),
            crate::generator::Color::Brown => image::Rgb([190, 99, 9]),
            crate::generator::Color::Black => image::Rgb([0, 0, 0]),
            crate::generator::Color::Purple => image::Rgb([220, 187, 252]),
            crate::generator::Color::Yellow => image::Rgb([254, 255, 41]),
        }
    }

    pub fn render<W: std::io::Write + std::io::Seek>(
        &mut self,
        mut w: W,
        canva: crate::generator::Canva,
    ) -> Result<(), crate::error::Error> {
        let pixel_size = 50;
        let margin = pixel_size / 2;
        let image_size = (pixel_size * canva.size() + (margin * 2)) as u32;

        let pixel_size = pixel_size as u32;
        let margin = margin as i32;
        let color = Self::rgb_color(canva.color());

        let mut img = image::RgbImage::new(image_size, image_size);
        let rect = imageproc::rect::Rect::at(0, 0).of_size(image_size, image_size);
        imageproc::drawing::draw_filled_rect_mut(&mut img, rect, image::Rgb([255, 255, 255]));

        canva
            .into_iter()
            .filter(|(_, displayed)| *displayed)
            .map(|(pt, _)| pt)
            .for_each(|pt| {
                let x = margin + (pt.x as i32 * pixel_size as i32);
                let y = margin + (pt.y as i32 * pixel_size as i32);
                let rect = imageproc::rect::Rect::at(x, y).of_size(pixel_size, pixel_size);
                imageproc::drawing::draw_filled_rect_mut(&mut img, rect, color);
            });

        img.write_to(&mut w, image::ImageOutputFormat::Png)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn terminal_render() {
        let generator: crate::generator::Seed = "hello".into();
        let canva = crate::generator::Canva::new(5, generator);
        let mut buffer = Vec::new();

        super::Terminal.render(&mut buffer, canva);

        let output = String::from_utf8(buffer).expect("failed to cast bytes to string");

        let expect = include_str!("../testdata/terminal_render.ascii");
        assert_eq!(expect, output)
    }

    #[test]
    fn png_render() {
        let generator: crate::generator::Seed = "hello".into();
        let canva = crate::generator::Canva::new(5, generator);
        let mut buffer = Vec::new();

        super::Png
            .render(std::io::Cursor::new(&mut buffer), canva)
            .expect("failed to render PNG");

        let expect = std::fs::read("testdata/png_render.png").expect("failed to read expected PNG");

        assert_eq!(expect, buffer)
    }
}
