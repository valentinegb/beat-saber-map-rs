//! Read and write Beat Saber maps with Rust.
//!
//! Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for language-agnostic documentation.

#![warn(missing_docs)]

mod hex;
pub mod info;

use std::{fs, path::PathBuf};

use serde::de::Error;

pub use self::info::Info;

/// Structural representation of a Beat Saber map folder.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for language-agnostic documentation.
#[derive(Debug, PartialEq, Default)]
pub struct BeatSaberMap {
    /// `Info.dat` file.
    ///
    /// See [`Info`].
    pub info: Info,
}

impl BeatSaberMap {
    /// Deserializes the files in a map folder.
    pub fn from_dir(dir: impl Into<PathBuf>) -> serde_json::Result<Self> {
        Ok(BeatSaberMap {
            info: serde_json::from_str(&fs::read_to_string(dir.into().join("Info.dat")).map_err(
                |err| {
                    let err_string = err.to_string();

                    serde_json::Error::custom(match err_string.chars().nth(0) {
                        Some(first_char) => {
                            first_char.to_lowercase().to_string() + &err_string[1..]
                        }
                        None => err_string,
                    })
                },
            )?)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dir_doesnt_fail() {
        BeatSaberMap::from_dir("sample").unwrap();
    }
}
