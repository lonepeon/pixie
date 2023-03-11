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

pub struct Canva {
    size: usize,
    canva: Vec<bool>,
}

impl Canva {
    pub fn new(size: usize, seed: Seed) -> Self {
        Self {
            size,
            canva: seed.take(size * size).collect(),
        }
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
        let generator: super::Seed = "hello".into();
        let canva = super::Canva::new(5, generator);

        assert_eq!(5, canva.size);
        assert_eq!(25, canva.canva.len());
    }

    #[test]
    fn canva_pixel() {
        let canva = super::Canva {
            size: 3,
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
