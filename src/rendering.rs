pub struct Terminal;

impl Terminal {
    pub fn render<W: std::io::Write>(
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

pub struct Png;

impl Png {
    pub fn render<W: std::io::Write + std::io::Seek>(
        &mut self,
        mut w: W,
        canva: crate::generator::Canva,
    ) -> Result<(), String> {
        let pixel_size = 50;
        let image_size = (pixel_size * canva.size()) as u32;

        let pixel_size = pixel_size as u32;
        let color = image::Rgb([0, 0, 0]);

        let mut img = image::RgbImage::new(image_size, image_size);
        let rect = imageproc::rect::Rect::at(0, 0).of_size(image_size, image_size);
        imageproc::drawing::draw_filled_rect_mut(&mut img, rect, image::Rgb([255, 255, 255]));

        canva
            .into_iter()
            .filter(|(_, displayed)| *displayed)
            .map(|(pt, _)| pt)
            .for_each(|pt| {
                let x = pt.x as i32 * pixel_size as i32;
                let y = pt.y as i32 * pixel_size as i32;
                let rect = imageproc::rect::Rect::at(x, y).of_size(pixel_size, pixel_size);
                imageproc::drawing::draw_filled_rect_mut(&mut img, rect, color);
            });

        img.write_to(&mut w, image::ImageOutputFormat::Png)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
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
