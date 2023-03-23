#[derive(Debug)]
pub(crate) enum AutumnErrors {
    FileNotSpecified,
    SampleTypeNotSupported(cpal::SampleFormat),
}

impl std::fmt::Display for AutumnErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotSpecified => write!(f, "File not specified"),
            Self::SampleTypeNotSupported(_) => write!(f, "Sample type not supported"),
        }
    }
}

impl std::error::Error for AutumnErrors {}