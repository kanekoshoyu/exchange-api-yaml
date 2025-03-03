use eyre::eyre;
use eyre::Result;
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::{Add, AddAssign};
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use strum_macros::{Display, EnumIter, EnumString};

pub trait GetVersion {
    fn get_version(&self) -> Option<Version>;
}
#[derive(Deserialize, Debug, Clone)]
pub struct AsyncApi {
    pub info: AsyncApiInfo,
}
impl GetVersion for AsyncApi {
    fn get_version(&self) -> Option<Version> {
        self.info.version
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AsyncApiInfo {
    pub version: Option<Version>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OpenApi {
    pub info: OpenApiInfo,
}
impl GetVersion for OpenApi {
    fn get_version(&self) -> Option<Version> {
        self.info.version
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct OpenApiInfo {
    pub version: Option<Version>,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    Serialize,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
pub enum ApiFileFormat {
    #[strum(serialize = "openapi")]
    OpenApi,
    #[strum(serialize = "asyncapi")]
    AsyncApi,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    Serialize,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
pub enum Protocol {
    #[strum(serialize = "rest")]
    Rest,
    #[strum(serialize = "ws")]
    Ws,
    #[strum(serialize = "fix")]
    Fix,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}
impl Version {
    pub fn current_crate() -> Result<Self> {
        let version_str = env!("CARGO_PKG_VERSION");
        Version::from_str(version_str)
    }
}
impl Add for Version {
    type Output = Version;

    fn add(self, rhs: Self) -> Self::Output {
        Version {
            major: self.major + rhs.major,
            minor: self.minor + rhs.minor,
            patch: self.patch + rhs.patch,
        }
    }
}
impl AddAssign for Version {
    fn add_assign(&mut self, rhs: Self) {
        self.major += rhs.major;
        self.minor += rhs.minor;
        self.patch += rhs.patch;
    }
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the version string by dots
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            // Return an error if the format is incorrect
            return Err(eyre!("invalid version format"));
        }

        // Parse each part into an integer, propagating any errors
        let major = parts[0].parse::<usize>()?;
        let minor = parts[1].parse::<usize>()?;
        let patch = parts[2].parse::<usize>()?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a custom visitor to handle deserialization
        struct VersionVisitor;

        impl serde::de::Visitor<'_> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a version string in the format 'major.minor.patch'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Split the version string by dots
                let parts: Vec<&str> = v.split('.').collect();
                if parts.len() != 3 {
                    return Err(serde::de::Error::custom("invalid version string format"));
                }

                // Parse each part into an integer
                let major = parts[0]
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)?;
                let minor = parts[1]
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)?;
                let patch = parts[2]
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)?;

                Ok(Version {
                    major,
                    minor,
                    patch,
                })
            }
        }

        // Use the custom visitor to deserialize the string
        deserializer.deserialize_str(VersionVisitor)
    }
}

/// input file parameter
pub struct InputFileParameter {
    /// we keep adding exchanges, no pub enum
    pub exchange: String,
    pub protocol: Protocol,
    pub format: ApiFileFormat,
    pub version: Version,
    pub filename: PathBuf,
}
impl InputFileParameter {
    /// return InputFileParameter from a filename
    pub fn from_file_path(filename: impl AsRef<Path>) -> Result<Self> {
        let filename = filename.as_ref();
        let filename_pathbuf = filename.to_path_buf();

        if !filename.is_file() {
            return Err(eyre::eyre!("file does not exist, {filename:?}"));
        }

        // read content
        let file_content = std::fs::read_to_string(filename).expect("Failed to read YAML file");

        // "binance_ws_asyncapi.yaml"
        let filename = filename.file_name().unwrap();
        let filename = filename.to_str().unwrap();

        // "binance_ws_asyncapi"
        if !filename.contains(".yaml") {
            return Err(eyre::eyre!("config file is not conistent"));
        }
        let rest = filename.to_string().replace(".yaml", "");

        // "binance", "ws", "asyncapi"
        let str_vec: Vec<&str> = rest.split("_").collect();

        if str_vec.len() != 3 {
            return Err(eyre::eyre!("invalid format"));
        }
        let exchange = str_vec[0].to_string();
        let protocol = Protocol::from_str(str_vec[1])?;
        let format = ApiFileFormat::from_str(str_vec[2])?;

        let version = match format {
            ApiFileFormat::OpenApi => {
                let info: OpenApi =
                    serde_yaml::from_str(&file_content).expect("Failed to parse YAML");
                info.get_version()
            }
            ApiFileFormat::AsyncApi => {
                let info: AsyncApi =
                    serde_yaml::from_str(&file_content).expect("Failed to parse YAML");
                info.get_version()
            }
        }
        .unwrap_or_default();

        Ok(InputFileParameter {
            exchange,
            protocol,
            format,
            version,
            filename: filename_pathbuf,
        })
    }

    /// returns a list of InputFileParameter from directory containing config files
    pub fn from_directory(directory: impl AsRef<Path>) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        let directory = directory.as_ref();
        let files = std::fs::read_dir(directory).unwrap();
        for file in files {
            let file_name = file?.file_name();
            let file_name = file_name
                .into_string()
                .map_err(|_| eyre!("file name format"))?;
            // filter out metadata files like DS_STORE
            if file_name.contains(".yaml") {
                let file_path = directory.join(file_name);
                let file = InputFileParameter::from_file_path(file_path)?;
                result.push(file);
            }
        }
        Ok(result)
    }
}
