use sha2::Digest;

pub struct Generator {
    data: Vec<u8>,
    position: usize,
}

impl<R: AsRef<str>> From<R> for Generator {
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

impl Iterator for Generator {
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

#[cfg(test)]
mod tests {
    #[test]
    fn load_from_string() {
        let generator: super::Generator = "hello".into();
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
    fn iterator() {
        let generator = super::Generator {
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
}
