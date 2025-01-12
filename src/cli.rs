use clap::Parser;
use std::path::PathBuf;

const CSV_FILE_SUFFIX: &str = "csv";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File path for the file to read into memory
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Sequence ID for the MixMax Sequence to append names to
    #[arg(short, long)]
    pub sequence_id: String,
}

impl Args {
    pub fn is_valid(&self) -> bool {
        // Validate file path
        let Some(ext) = self.file_path.extension() else {
            return false;
        };
        let file_path_valid =
            self.file_path.exists() && !self.file_path.is_dir() && ext == CSV_FILE_SUFFIX;
        if !file_path_valid {
            return false;
        }

        // Validate the sequence ID
        if self.sequence_id.is_empty() {
            return false;
        }

        true
    }
}
