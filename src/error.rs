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
                    "file is not readable/writeable".to_string()
                }
                err => format!("{}", err),
            },
        };

        if let Some(ctx) = &self.context {
            write!(f, "{}: ", ctx)?;
        }

        write!(f, "{}", message)
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

#[cfg(test)]
mod tests {
    #[test]
    fn error_display_generic_without_context() {
        let err = super::Error {
            kind: super::Kind::Generic("my error".to_string()),
            context: None,
        };

        assert_eq!("my error".to_string(), format!("{}", err))
    }

    #[test]
    fn error_display_generic_with_context() {
        let err = super::Error {
            kind: super::Kind::Generic("my error".to_string()),
            context: None,
        }
        .context("a bit of context".to_string());

        assert_eq!("a bit of context: my error".to_string(), format!("{}", err))
    }

    #[test]
    fn error_display_io_permission_denied_without_context() {
        let err = super::Error {
            kind: super::Kind::IoError(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "permission denied",
            )),
            context: None,
        };

        assert_eq!(
            "file is not readable/writeable".to_string(),
            format!("{}", err)
        )
    }
    #[test]
    fn error_display_io_permission_denied_with_context() {
        let err = super::Error {
            kind: super::Kind::IoError(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "permission denied",
            )),
            context: None,
        }
        .context("my-file.txt".to_string());

        assert_eq!(
            "my-file.txt: file is not readable/writeable".to_string(),
            format!("{}", err)
        )
    }

    #[test]
    fn error_display_io_without_context() {
        let err = super::Error {
            kind: super::Kind::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            )),
            context: None,
        };

        assert_eq!("entity not found".to_string(), format!("{}", err))
    }

    #[test]
    fn error_display_io_with_context() {
        let err = super::Error {
            kind: super::Kind::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            )),
            context: None,
        }
        .context("my-file.txt".to_string());

        assert_eq!(
            "my-file.txt: entity not found".to_string(),
            format!("{}", err)
        )
    }

    #[test]
    fn error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = super::Error::from(io_err);
        let super::Kind::IoError(wrapped_err) = err.kind else { panic!("unexpected kind" )};

        assert_eq!(std::io::ErrorKind::NotFound, wrapped_err.kind());
        assert_eq!(None, err.context);
    }

    #[test]
    fn error_from_image_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let img_err = image::error::ImageError::IoError(io_err);
        let err = super::Error::from(img_err);
        let super::Kind::IoError(wrapped_err) = err.kind else { panic!("unexpected kind" )};

        assert_eq!(std::io::ErrorKind::NotFound, wrapped_err.kind());
        assert_eq!(None, err.context);
    }

    #[test]
    fn error_from_image_error() {
        let inner_image_err = image::error::UnsupportedError::from_format_and_kind(
            image::error::ImageFormatHint::Unknown,
            image::error::UnsupportedErrorKind::Format(image::error::ImageFormatHint::Unknown),
        );
        let img_err = image::error::ImageError::Unsupported(inner_image_err);
        let err = super::Error::from(img_err);
        let super::Kind::Generic(msg) = err.kind else { panic!("unexpected kind" )};

        assert_eq!("The image format could not be determined", msg);
        assert_eq!(None, err.context);
    }
}
