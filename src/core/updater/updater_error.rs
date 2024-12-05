use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub enum UpdaterErrorKind {
    NotAnAngularProject,
    FailedToExtractI18n
}

#[derive(Clone, Debug)]
pub struct UpdaterError {
    pub kind: UpdaterErrorKind,
}

impl UpdaterError {
    pub fn new(kind: UpdaterErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for UpdaterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.kind {
            UpdaterErrorKind::NotAnAngularProject => {
                "Current working directory is not an Anuglar project. (Could not find angular.json)"
            }
            UpdaterErrorKind::FailedToExtractI18n => {
                "Failed to extract i18n file via ng extract-i18n. Is @angular/cli installed?"
            }
        })
    }
}

impl Error for UpdaterError {}
