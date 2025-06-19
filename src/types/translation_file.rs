use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct TranslationFile {
    locale: String,
    translations: BTreeMap<String, String>,
}

impl TranslationFile {
    pub fn extend_with(&mut self, other: &TranslationFile) {
        for (key, text) in &other.translations {
            if self.translations.contains_key(key) {
                continue;
            }

            self.translations.insert(key.clone(), text.clone());
        }
    }

    pub fn reduce_to(&mut self, other: &TranslationFile) {
        let target_keys: Vec<String> = self
            .translations
            .keys()
            .filter(|key| !other.translations.contains_key(*key))
            .cloned()
            .collect();

        for key in &target_keys {
            self.translations.remove(key);
        }
    }

    pub fn try_from_path(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new().read(true).open(path)?;

        Ok(serde_json::from_reader(BufReader::new(file))?)
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        serde_json::to_writer_pretty(BufWriter::new(file), self)?;

        Ok(())
    }

    pub fn write_or_extend(&self, path: &PathBuf, enforce_parity: bool) -> Result<(), Box<dyn Error>> {
        match Self::try_from_path(path) {
            Ok(mut other) => {
                other.extend_with(self);
                if enforce_parity {
                    other.reduce_to(self);
                }
                other.write(path)?;
            }
            Err(_) => {
                self.write(path)?;
            }
        }

        Ok(())
    }
}

impl Default for TranslationFile {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            translations: BTreeMap::new(),
        }
    }
}
