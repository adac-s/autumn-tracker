use crate::errors;

pub(crate) struct WavSample {
    pub filename: std::path::PathBuf,
    pub data: Vec<i16>,
}

impl WavSample {
    pub(crate) fn new_from_str(filename: &str) -> eyre::Result<Self> {
        let path = filename.into();
        let mut reader = hound::WavReader::open(&path)?;
        let samples: Vec<i16> = reader.samples::<i16>().map(|x| x).collect::<Result<Vec<i16>, hound::Error>>()?;

        Ok(Self {
            filename: path,
            data: samples,
        })
    }

    pub(crate) fn new() -> eyre::Result<Self> {
        let res = rfd::FileDialog::new()
            .add_filter("Audio (wav)", &["wav"])
            .pick_file();

        if let Some(p) = res {
            let mut reader = hound::WavReader::open(&p)?;
            let samples: Vec<i16> = reader.samples::<i16>().map(|x| x).collect::<Result<Vec<i16>, hound::Error>>()?;
            Ok(Self {
                filename: p,
                data: samples,
            })
        }
        else {
            Err(errors::AutumnErrors::FileNotSpecified.into())
        }
    }
}