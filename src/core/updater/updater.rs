use crate::core::updater::updater_error::UpdaterError;
use crate::core::updater::updater_error::UpdaterErrorKind::{
    FailedToExtractI18n, NotAnAngularProject,
};
use crate::types::args::UpdaterArgs;
use crate::types::translation_file::TranslationFile;
use regex::Regex;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub struct Updater {
    config: UpdaterArgs,
}

impl Updater {
    pub fn new(config: UpdaterArgs) -> Self {
        Self { config }
    }

    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        Self::check_for_root()?;

        let source_path = self
            .config
            .source_path
            .clone()
            .unwrap_or("./i18n/messages.json".into());

        if !self.config.no_extract {
            let mut i18n_dir_path = PathBuf::from(&source_path);

            i18n_dir_path.pop();

            let i18n_dir_path = i18n_dir_path.to_string_lossy().to_string();

            Self::extract_i18n(&i18n_dir_path)?;
        }

        if !self.config.no_sort {
            Self::sort_and_write_source(&source_path)?;
        }

        let i18n_paths = Self::get_i18n_paths(&source_path, &self.config.target_languages)?;

        Self::update_i18n(&source_path, &i18n_paths)?;

        Ok(())
    }

    fn check_for_root() -> Result<(), Box<dyn Error>> {
        if !PathBuf::from("angular.json").is_file() {
            return Err(Box::new(UpdaterError::new(NotAnAngularProject)));
        }

        Ok(())
    }

    fn extract_i18n(output_path: &str) -> Result<String, Box<dyn Error>> {
        let out = Command::new("ng")
            .arg("extract-i18n")
            .args(["--format", "json"])
            .args(["--output-path", output_path])
            .output()?;

        if !out.status.success() {
            return Err(Box::new(UpdaterError::new(FailedToExtractI18n)));
        }

        Ok("i18n/messages.json".into())
    }

    fn get_i18n_paths(
        source_path: &String,
        target_languages: &Option<Vec<String>>,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        match target_languages {
            Some(target_languages) => {
                let mut i18n_paths: Vec<PathBuf> = Vec::new();

                for language in target_languages {
                    let mut path = source_path.clone();

                    let Some(index) = source_path.rfind('.') else {
                        continue;
                    };

                    path.insert_str(index, &format!(".{language}"));

                    i18n_paths.push(PathBuf::from(path));
                }

                Ok(i18n_paths)
            }
            None => {
                let matcher = Regex::new(r#"\..+\.json$"#).expect("Invalid regex in source code");
                let read_dir = PathBuf::from(source_path).parent().unwrap().read_dir()?;

                let entries: Vec<PathBuf> = read_dir
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| matcher.is_match(&entry.path().to_string_lossy()))
                    .map(|entry| entry.path())
                    .collect();

                Ok(entries)
            }
        }
    }

    fn update_i18n(source_path: &String, i18n_paths: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
        let source_path = PathBuf::from(source_path);
        let source = TranslationFile::try_from_path(&source_path)?;

        for i18n_path in i18n_paths {
            source.write_or_extend(i18n_path, true)?
        }

        Ok(())
    }

    fn sort_and_write_source(source_path: &String) -> Result<(), Box<dyn Error>> {
        let source_path = PathBuf::from(source_path);
        let source = TranslationFile::try_from_path(&source_path)?;
        source.write(&source_path)?;

        Ok(())
    }
}
