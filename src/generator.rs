use sha2::Digest;

pub struct Seed {
    data: Vec<u8>,
    position: usize,
}

impl<R: AsRef<str>> From<R> for Seed {
    fn from(value: R) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(value.as_ref());
        let data = hasher.finalize();

        Self {
            data: data.to_vec(),
            position: 0,
        }
    }
}

impl Iterator for Seed {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.data.len() {
            self.position = 0;
        }

        let value = self.data[self.position];
        self.position += 1;

        Some(value % 2 == 0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Brown,
    Green,
    Pink,
    Purple,
    Red,
    Yellow,
}

impl Color {
    pub fn new<S: Iterator<Item = bool>>(seed: S) -> Self {
        let number_of_colors = 8;
        let value: usize = seed.take(10 * number_of_colors).map(|b| b as usize).sum();

        match value % number_of_colors {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Purple,
            4 => Self::Pink,
            5 => Self::Brown,
            6 => Self::Yellow,
            _ => Self::Black,
        }
    }
}

#[derive(Clone)]
pub struct Canva {
    color: Color,
    size: usize,
    canva: Vec<bool>,
}

impl Canva {
    pub fn new<S: Iterator<Item = bool>>(size: usize, mut seed: S) -> Self {
        let middle = size / 2;
        let mut canva = vec![false; size * size];

        let left_side: Vec<usize> = canva
            .iter()
            .enumerate()
            .filter(|(index, _)| index % size < middle)
            .map(|(index, _)| index)
            .collect();

        left_side.into_iter().for_each(|index| {
            let mirrored_index = (size * (index / size)) + (size - 1) - (index % size);
            let value = seed
                .next()
                .expect("seeder should be infinite and always yield values");

            canva[index] = value;
            canva[mirrored_index] = value;
        });

        let color = Color::new(seed);

        Self { size, color, canva }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn pixel(&self, pt: Point) -> Option<bool> {
        if pt.x >= self.size || pt.y >= self.size {
            return None;
        }
        let index = pt.x + (self.size * pt.y);

        Some(self.canva[index])
    }
}

impl IntoIterator for Canva {
    type Item = (Point, bool);
    type IntoIter = CanvaIter;

    fn into_iter(self) -> Self::IntoIter {
        CanvaIter {
            canva: self,
            position: Point { x: 0, y: 0 },
        }
    }
}

pub struct CanvaIter {
    canva: Canva,
    position: Point,
}

impl Iterator for CanvaIter {
    type Item = (Point, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let current_position = self.position;
        self.position = if self.position.x + 1 == self.canva.size() {
            Point {
                x: 0,
                y: self.position.y + 1,
            }
        } else {
            Point {
                x: self.position.x + 1,
                ..self.position
            }
        };

        self.canva
            .pixel(current_position)
            .map(|pixel| (current_position, pixel))
    }
}

#[cfg(test)]
mod tests {
    struct StaticSeeder {
        data: Vec<bool>,
        position: usize,
    }

    impl Iterator for StaticSeeder {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.position == self.data.len() {
                self.position = 0;
            }
            let value = self.data[self.position];
            self.position += 1;
            Some(value)
        }
    }

    #[test]
    fn seed_from_string() {
        let generator: super::Seed = "hello".into();
        assert_eq!(0, generator.position);
        assert_eq!(
            vec![
                44, 242, 77, 186, 95, 176, 163, 14, 38, 232, 59, 42, 197, 185, 226, 158, 27, 22,
                30, 92, 31, 167, 66, 94, 115, 4, 51, 98, 147, 139, 152, 36
            ],
            generator.data
        );
    }

    #[test]
    fn seed_iterator() {
        let generator = super::Seed {
            position: 0,
            data: vec![12, 13, 240, 4],
        };

        let mut iter = generator;
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(true), iter.next()); // rollover
        assert_eq!(Some(false), iter.next());
    }

    #[test]
    fn canva_new() {
        let generator = StaticSeeder {
            data: vec![false, true, false, true, false, true],
            position: 0,
        };
        let canva = super::Canva::new(6, generator);

        assert_eq!(6, canva.size);
        assert_eq!(super::Color::Red, canva.color);
        assert_eq!(36, canva.canva.len());
        assert_eq!(
            vec![
                false, true, false, false, true, false, true, false, true, true, false, true,
                false, true, false, false, true, false, true, false, true, true, false, true,
                false, true, false, false, true, false, true, false, true, true, false, true,
            ],
            canva.canva
        )
    }

    #[test]
    fn canva_pixel() {
        let canva = super::Canva {
            size: 3,
            color: super::Color::Black,
            canva: vec![false, true, false, true, false, true, false, true, false],
        };

        assert_eq!(Some(false), canva.pixel(super::Point { x: 0, y: 0 }));
        assert_eq!(Some(true), canva.pixel(super::Point { x: 1, y: 0 }));
        assert_eq!(Some(false), canva.pixel(super::Point { x: 2, y: 0 }));
        assert_eq!(Some(true), canva.pixel(super::Point { x: 0, y: 1 }));
        assert_eq!(Some(false), canva.pixel(super::Point { x: 1, y: 1 }));
        assert_eq!(Some(true), canva.pixel(super::Point { x: 2, y: 1 }));
        assert_eq!(Some(false), canva.pixel(super::Point { x: 0, y: 2 }));
        assert_eq!(Some(true), canva.pixel(super::Point { x: 1, y: 2 }));
        assert_eq!(Some(false), canva.pixel(super::Point { x: 2, y: 2 }));
        assert_eq!(None, canva.pixel(super::Point { x: 2, y: 3 }));
        assert_eq!(None, canva.pixel(super::Point { x: 4, y: 1 }));
    }

    #[test]
    fn canva_iterator() {
        let canva = super::Canva {
            size: 3,
            color: super::Color::Black,
            canva: vec![false, true, false, true, false, true, false, true, false],
        };

        let mut iter = canva.into_iter();

        assert_eq!(Some((super::Point { x: 0, y: 0 }, false)), iter.next());
        assert_eq!(Some((super::Point { x: 1, y: 0 }, true)), iter.next());
        assert_eq!(Some((super::Point { x: 2, y: 0 }, false)), iter.next());
        assert_eq!(Some((super::Point { x: 0, y: 1 }, true)), iter.next());
        assert_eq!(Some((super::Point { x: 1, y: 1 }, false)), iter.next());
        assert_eq!(Some((super::Point { x: 2, y: 1 }, true)), iter.next());
        assert_eq!(Some((super::Point { x: 0, y: 2 }, false)), iter.next());
        assert_eq!(Some((super::Point { x: 1, y: 2 }, true)), iter.next());
        assert_eq!(Some((super::Point { x: 2, y: 2 }, false)), iter.next());
        assert_eq!(None, iter.next());
    }
}
