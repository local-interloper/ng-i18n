use std::{error::Error, path::PathBuf};

use crate::types::{args::MergerArgs, i18n_file::I18nFile};

pub struct Merger {
    args: MergerArgs,
}

impl Merger {
    pub fn new(args: MergerArgs) -> Self {
        Merger { args }
    }

    pub fn merge(&mut self) -> Result<(), Box<dyn Error>> {
        let mut complete = I18nFile::default();

        for path in &self.args.target_files {
            let Ok(other) = &I18nFile::try_from_path(path) else {
                continue;
            };

            complete.extend_with(other);
        }

        let output_path = PathBuf::from(
            self.args
                .output_path
                .as_ref()
                .unwrap_or(&"messages.json".to_string()),
        );

        complete.write(&output_path)?;

        Ok(())
    }
}
