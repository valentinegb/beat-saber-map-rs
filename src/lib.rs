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
//! Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for
//! language-agnostic documentation.

#![warn(missing_docs)]

pub mod audio;
pub mod beatmap;
mod hex;
pub mod info;

use std::{io, path::Path};

use thiserror::Error;

pub use self::{audio::Audio, beatmap::Beatmap, info::Info};

/// Any error that may occur from a function originating in this library.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from [`serde_json`].
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    /// Error from [`std::io`].
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Occurs when failing to convert [`u8`] to [`beatmap::LineIndex`].
    #[error("Could not convert u8 to LineIndex, expected integer from 0 to 3, got {0}")]
    LineIndexTryFromU8(u8),
    /// Occurs when failing to convert [`u8`] to [`beatmap::LineLayer`].
    #[error("Could not convert u8 to LineLayer, expected integer from 0 to 2, got {0}")]
    LineLayerTryFromU8(u8),
    /// Occurs when failing to convert [`u8`] to [`beatmap::Color`].
    #[error("Could not convert u8 to Color, expected 0 or 1, got {0}")]
    ColorTryFromU8(u8),
    /// Occurs when failing to convert [`u8`] to [`beatmap::CutDirection`].
    #[error("Could not convert u8 to CutDirection, expected integer from 0 to 8, got {0}")]
    CutDirectionTryFromU8(u8),
}

/// Structural representation of a Beat Saber map folder.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for
/// language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default)]
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
        let info = Info::from_file(dir.as_ref().join("Info.dat"))?;

        Ok(BeatSaberMap {
            audio: Audio::from_file(dir.as_ref().join(&info.audio.audio_data_filename))?,
            info,
        })
    }
}
