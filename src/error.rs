use dialoguer::Error as DialoguerError;

use mago_php_version::PHPVersion;
use mago_php_version::error::ParsingError;
use mago_reporting::error::ReportingError;
use mago_source::error::SourceError;

#[derive(Debug)]
pub enum Error {
    Source(SourceError),
    Reporting(ReportingError),
    BuildingRuntime(std::io::Error),
    Walking(async_walkdir::Error),
    BuildingConfiguration(config::ConfigError),
    DeserializingToml(toml::de::Error),
    SerializingToml(toml::ser::Error),
    CanonicalizingPath(std::path::PathBuf, std::io::Error),
    Join(tokio::task::JoinError),
    Json(serde_json::Error),
    SelfUpdate(self_update::errors::Error),
    PHPVersionIsTooOld(PHPVersion, PHPVersion),
    PHPVersionIsTooNew(PHPVersion, PHPVersion),
    InvalidPHPVersion(String, ParsingError),
    Dialoguer(DialoguerError),
    WritingConfiguration(std::io::Error),
    ReadingComposerJson(std::io::Error),
    ParsingComposerJson(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source(error) => write!(f, "Failed to load source files: {error}"),
            Self::Reporting(error) => write!(f, "Failed to report results: {error}"),
            Self::Walking(error) => write!(f, "Failed to walk the source tree: {error}"),
            Self::BuildingRuntime(error) => write!(f, "Failed to build the runtime: {error}"),
            Self::BuildingConfiguration(error) => write!(f, "Failed to build the configuration: {error}"),
            Self::DeserializingToml(error) => write!(f, "Failed to deserialize TOML: {error}"),
            Self::SerializingToml(error) => write!(f, "Failed to serialize TOML: {error}"),
            Self::CanonicalizingPath(path, error) => write!(f, "Failed to canonicalize path `{path:?}`: {error}"),
            Self::Join(error) => write!(f, "Failed to join tasks: {error}"),
            Self::Json(error) => write!(f, "Failed to parse JSON: {error}"),
            Self::SelfUpdate(error) => write!(f, "Failed to self update: {error}"),
            Self::PHPVersionIsTooOld(minimum, actual) => {
                write!(f, "PHP version {actual} is not supported, minimum supported version is {minimum}")
            }
            Self::PHPVersionIsTooNew(maximum, actual) => {
                write!(f, "PHP version {actual} is not supported, maximum supported version is {maximum}")
            }
            Self::InvalidPHPVersion(version, error) => {
                write!(f, "Invalid PHP version `{version}`: {error}")
            }
            Self::Dialoguer(error) => write!(f, "Failed to interact with the user: {error}"),
            Self::WritingConfiguration(error) => write!(f, "Failed to write the configuration file: {error}"),
            Self::ReadingComposerJson(error) => write!(f, "Failed to read the `composer.json` file: {error}"),
            Self::ParsingComposerJson(error) => write!(f, "Failed to parse the `composer.json` file: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Source(error) => Some(error),
            Self::Reporting(error) => Some(error),
            Self::Walking(error) => Some(error),
            Self::BuildingConfiguration(error) => Some(error),
            Self::BuildingRuntime(error) => Some(error),
            Self::DeserializingToml(error) => Some(error),
            Self::SerializingToml(error) => Some(error),
            Self::CanonicalizingPath(_, error) => Some(error),
            Self::Join(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::SelfUpdate(error) => Some(error),
            Self::InvalidPHPVersion(_, error) => Some(error),
            Self::Dialoguer(error) => Some(error),
            Self::WritingConfiguration(error) => Some(error),
            Self::ReadingComposerJson(error) => Some(error),
            Self::ParsingComposerJson(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SourceError> for Error {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

impl From<ReportingError> for Error {
    fn from(error: ReportingError) -> Self {
        Self::Reporting(error)
    }
}

impl From<async_walkdir::Error> for Error {
    fn from(error: async_walkdir::Error) -> Self {
        Self::Walking(error)
    }
}

impl From<config::ConfigError> for Error {
    fn from(error: config::ConfigError) -> Self {
        Self::BuildingConfiguration(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::DeserializingToml(error)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::SerializingToml(error)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(error: tokio::task::JoinError) -> Self {
        Self::Join(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<self_update::errors::Error> for Error {
    fn from(error: self_update::errors::Error) -> Self {
        Self::SelfUpdate(error)
    }
}

impl From<DialoguerError> for Error {
    fn from(error: DialoguerError) -> Self {
        Self::Dialoguer(error)
    }
}
