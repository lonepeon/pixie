#[derive(Debug)]
pub struct Error {
    context: Option<String>,
    kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    Generic(String),
    IoError(std::io::Error),
}

impl std::error::Error for Error {}

impl Error {
    pub fn context(mut self, msg: String) -> Self {
        self.context = Some(msg);
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self.kind {
            Kind::Generic(msg) => msg.to_string(),
            Kind::IoError(err) => match &err.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    "file is not readabale/wrieable".to_string()
                }
                err => format!("{}", err),
            },
        };

        if let Some(ctx) = &self.context {
            write!(f, "{}: ", ctx)?;
        }

        writeln!(f, "{} ", message)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            context: None,
            kind: Kind::IoError(value),
        }
    }
}

impl From<image::error::ImageError> for Error {
    fn from(value: image::error::ImageError) -> Self {
        let kind = match value {
            image::error::ImageError::IoError(err) => Kind::IoError(err),
            err => Kind::Generic(err.to_string()),
        };

        Self {
            context: None,
            kind,
        }
    }
}
