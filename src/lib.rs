//! Read and write Beat Saber maps with Rust.
//!
//! ```
//! use beat_saber_map::BeatSaberMap;
//!
//! let map = BeatSaberMap::from_dir("sample").unwrap();
//!
//! assert_eq!(map.info.version, "4.0.0");
//! assert_eq!(map.info.song.title, "Magic");
//! assert_eq!(map.info.environment_names.first().unwrap(), "WeaveEnvironment");
//! ```

#![warn(missing_docs)]

/// Generates a message that refers a reader to a page of the BSMG Wiki.
///
/// The first argument is expected to be a string literal of a subpage of the
/// `map-format` page. If not provided, gives a link to `map-format.html`
/// itself. To include an anchor that takes the reader to a specific section of
/// the page, put `#` followed by a string literal.
///
/// # Examples
///
/// ```ignore
/// /// Takes you to the main page (`map-format.html`).
/// #[doc = bsmg_wiki!()]
/// pub struct MainPage;
///
/// /// Takes you to the checksum section on the main page (`map-format.html`).
/// #[doc = bsmg_wiki!(#"checksum")]
/// pub struct ChecksumSection;
///
/// /// Takes you to the info page.
/// #[doc = bsmg_wiki!("info")]
/// pub struct InfoPage;
///
/// /// Takes you to the summary section on the info page.
/// #[doc = bsmg_wiki!("info"#"summary")]
/// pub struct SummarySection;
/// ```
macro_rules! bsmg_wiki {
    ($($page:literal)?$(#$anchor:literal)?) => {
        concat!(
            "\n\nRefer to the [BSMG Wiki] for language-agnostic documentation.\n\n[BSMG Wiki]: https://bsmg.wiki/mapping/map-format",
            $(
                "/",
                $page,
            )?
            ".html",
            $(
                "#",
                $anchor,
            )?
        )
    };
}

/// Documentation for `version` fields.
///
/// # Examples
///
/// ```ignore
/// struct Structure {
///     #[doc = version_doc!()]
///     pub version: String,
///     // ...
/// }
/// ```
macro_rules! version_doc {
    () => {
        r#"Should be "4.0.0", that's the currently supported schema version."#
    };
}

#[macro_use]
pub mod audio;
#[macro_use]
pub mod beatmap;
mod hex;
#[macro_use]
pub mod info;
// #[macro_use]
// pub mod lightshow;

use std::{collections::HashMap, ffi::OsString, io, path::Path};

use thiserror::Error;

pub use self::{audio::Audio, beatmap::Beatmap, info::Info /* , lightshow::Lightshow */};

/// This type represents the beats of a song as a measurement of time.
pub type Beats = f64;

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
    /// Occurs when failing to convert [`u8`] to [`beatmap::MidAnchorMode`].
    #[error("Could not convert u8 to MidAnchorMode, expected integer from 0 to 2, got {0}")]
    MidAnchorModeTryFromU8(u8),
    /// Occurs when failing to convert [`u8`] to [`beatmap::ExecutionTime`].
    #[deprecated = "`beatmap::ExecutionTime` is deprecated"]
    #[error("Could not convert u8 to ExecutionTime, expected 0 or 1, got {0}")]
    ExecutionTimeTryFromU8(u8),
}

/// A structural representation of a Beat Saber map folder.
#[doc = bsmg_wiki!()]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BeatSaberMap {
    /// The `Info.dat` file.
    ///
    /// See [`Info`].
    pub info: Info,
    /// The `BPMInfo.dat` file.
    ///
    /// See [`Audio`].
    pub audio: Audio,
    /// Any beatmap files that may exist.
    ///
    /// See [`Beatmap`].
    pub beatmaps: HashMap<OsString, Beatmap>,
}

impl BeatSaberMap {
    /// Deserializes the files in a map folder.
    pub fn from_dir(dir: impl AsRef<Path>) -> Result<Self, Error> {
        let info = Info::from_file(dir.as_ref().join("Info.dat"))?;
        let mut beatmaps = HashMap::new();

        for beatmap in info.difficulty_beatmaps.iter() {
            beatmaps.insert(
                beatmap
                    .beatmap_data_filename
                    .file_stem()
                    .unwrap_or(beatmap.beatmap_data_filename.as_os_str())
                    .to_os_string(),
                Beatmap::from_file(dir.as_ref().join(&beatmap.beatmap_data_filename))?,
            );
        }

        Ok(BeatSaberMap {
            audio: Audio::from_file(dir.as_ref().join(&info.audio.audio_data_filename))?,
            info,
            beatmaps,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn contains_beatmaps() {
        let beatmaps = BeatSaberMap::from_dir("sample").unwrap().beatmaps;

        assert!(beatmaps.contains_key(&OsString::from_str("Easy").unwrap()));
        assert!(beatmaps.contains_key(&OsString::from_str("Normal").unwrap()));
        assert!(beatmaps.contains_key(&OsString::from_str("Hard").unwrap()));
        assert!(beatmaps.contains_key(&OsString::from_str("Expert").unwrap()));
        assert!(beatmaps.contains_key(&OsString::from_str("ExpertPlus").unwrap()));
        assert_eq!(beatmaps.len(), 5);
    }
}
