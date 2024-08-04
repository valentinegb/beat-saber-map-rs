//! Read and write Beat Saber maps with Rust.
//!
//! ```
//! use beat_saber_map::BeatSaberMap;
//!
//! let map = BeatSaberMap::from_dir("sample").unwrap();
//!
//! assert_eq!(map.info.song.title, "Magic");
//! ```
//!
//! Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for language-agnostic documentation.

#![warn(missing_docs)]

pub mod audio;
mod hex;
pub mod info;

use std::{io, path::Path};

use thiserror::Error;

pub use self::{audio::Audio, info::Info};

/// Any error that may occur from a function originating in this library.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from [`serde_json`].
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    /// Error from [`std::io`].
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Structural representation of a Beat Saber map folder.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for language-agnostic documentation.
#[derive(Debug, PartialEq, Default)]
pub struct BeatSaberMap {
    /// `Info.dat` file.
    ///
    /// See [`Info`].
    pub info: Info,
    /// `BPMInfo.dat` file.
    ///
    /// See [`Audio`].
    pub audio: Audio,
}

impl BeatSaberMap {
    /// Deserializes the files in a map folder.
    pub fn from_dir(dir: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(BeatSaberMap {
            info: Info::from_file(dir.as_ref().join("Info.dat"))?,
            audio: Audio::from_file(dir.as_ref().join("BPMInfo.dat"))?,
        })
    }
}
