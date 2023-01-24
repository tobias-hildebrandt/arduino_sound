use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    #[arg(id = "input", help = "Input ABC file path")]
    input_file: PathBuf,

    #[arg(short = 'o', long = "output", id = "output", help = "Output file path")]
    output_file: Option<PathBuf>,

    #[arg(value_enum, short = 'f', help = "Output file format")]
    format: Option<FileFormat>,

    #[arg(short = 'v', help = "Print verbose debug information")]
    verbose: bool
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum FileFormat {
    #[value(help = "Raw PCM")]
    Raw,
    #[value(help = "WAV")]
    Wav,
    #[value(help = "Play to speakers")]
    Play,
}

impl Args {
    pub fn output_file(&self) -> Result<&std::path::Path, anyhow::Error> {
        match &self.output_file {
            Some(o) => Ok(o),
            None => Err(anyhow::anyhow!("must specify output file")),
        }
    }

    pub fn input_file(&self) -> PathBuf {
        self.input_file.clone()
    }

    pub fn file_format(&self) -> Option<FileFormat> {
        match &self.format {
            Some(f) => Some(*f),
            None => None,
        }
    }
}
