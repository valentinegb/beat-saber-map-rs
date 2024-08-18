//! Module related to beatmap files.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::Error;

/// Defines collections and associated metadata for all *interactable* beatmap
/// items, such as notes and obstacles.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Beatmap {
    /// Should be "4.0.0", that's the currently supported schema version.
    pub version: String,
    /// See [`ColorNote`].
    pub color_notes: Vec<ColorNote>,
    /// See [`ColorNoteData`].
    pub color_notes_data: Vec<ColorNoteData>,
}

impl Default for Beatmap {
    fn default() -> Self {
        Self {
            version: "4.0.0".to_string(),
            color_notes: Default::default(),
            color_notes_data: Default::default(),
        }
    }
}

impl Beatmap {
    /// Instatiates a [`Beatmap`] from a beatmap file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}

/// Placement of color note.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ColorNote {
    /// Beat, specific point in time, as determined by
    /// [BPM](super::info::Audio::bpm) of song, when
    /// this object should reach player.
    #[serde(rename = "b")]
    pub beat: u64,
    /// Rotation lane, degree of rotation relative to player that this object
    /// should spawn from. This is typically reserved for beatmaps using
    /// `360Degree` or `90Degree` characteristic.
    #[serde(rename = "r")]
    pub rotation_lane: i16,
    /// Metadata index.
    #[serde(rename = "i")]
    pub metadata_index: usize,
}

/// Attributes of color note.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ColorNoteData {
    /// See [`LineIndex`].
    #[serde(rename = "x")]
    pub line_index: LineIndex,
    /// See [`LineLayer`].
    #[serde(rename = "y")]
    pub line_layer: LineLayer,
    /// See [`Color`].
    #[serde(rename = "x")]
    pub color: Color,
    /// See [`CutDirection`].
    #[serde(rename = "d")]
    pub cut_direction: CutDirection,
    /// Angle offset, applies counter-clockwise rotational offset to note's cut
    /// direction.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes-angle-offset)
    /// for language-agnostic documentation.
    #[serde(rename = "a")]
    pub angle_offset: i16,
}

/// Horizontal row where object should reside on grid.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes-line-index)
/// for language-agnostic documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum LineIndex {
    #[default]
    FarLeft,
    Left,
    Right,
    FarRight,
}

impl TryFrom<u8> for LineIndex {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FarLeft),
            1 => Ok(Self::Left),
            2 => Ok(Self::Right),
            3 => Ok(Self::FarRight),
            other => Err(Error::LineIndexTryFromU8(other)),
        }
    }
}

impl Into<u8> for LineIndex {
    fn into(self) -> u8 {
        self as u8
    }
}

/// Vertical column where object should reside on grid.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes-line-layer)
/// for language-agnostic documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum LineLayer {
    #[default]
    Bottom,
    Middle,
    Top,
}

impl TryFrom<u8> for LineLayer {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bottom),
            1 => Ok(Self::Middle),
            2 => Ok(Self::Top),
            other => Err(Error::LineLayerTryFromU8(other)),
        }
    }
}

impl Into<u8> for LineLayer {
    fn into(self) -> u8 {
        self as u8
    }
}

/// Which saber should be able to successfully cut note.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes-type)
/// for language-agnostic documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum Color {
    #[default]
    LeftSaber,
    RightSaber,
}

impl TryFrom<u8> for Color {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LeftSaber),
            1 => Ok(Self::RightSaber),
            other => Err(Error::ColorTryFromU8(other)),
        }
    }
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        self as u8
    }
}

/// Direction player should swing to successfully cut note.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes-cut-direction)
/// for language-agnostic documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum CutDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Any,
}

impl TryFrom<u8> for CutDirection {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Up),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Right),
            4 => Ok(Self::UpLeft),
            5 => Ok(Self::UpRight),
            6 => Ok(Self::DownLeft),
            7 => Ok(Self::DownRight),
            8 => Ok(Self::Any),
            other => Err(Error::CutDirectionTryFromU8(other)),
        }
    }
}

impl Into<u8> for CutDirection {
    fn into(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn sample() -> String {
        fs::read_to_string("sample/Normal.dat").unwrap()
    }

    fn manual_recreation() -> Beatmap {
        Beatmap {
            version: "4.0.0".to_string(),
            color_notes: vec![ColorNote {
                beat: 10,
                rotation_lane: 0,
                metadata_index: 0,
            }],
            color_notes_data: vec![
                ColorNoteData {
                    line_index: LineIndex::Left,
                    line_layer: LineLayer::Bottom,
                    color: Color::LeftSaber,
                    cut_direction: CutDirection::Down,
                    angle_offset: 0,
                },
                ColorNoteData {
                    line_index: LineIndex::Right,
                    line_layer: LineLayer::Top,
                    color: Color::LeftSaber,
                    cut_direction: CutDirection::Up,
                    angle_offset: 0,
                },
            ],
        }
    }

    #[test]
    #[ignore = "not all fields are implemented"]
    fn serializes_correctly() {
        let serialized = serde_json::to_string_pretty(&manual_recreation()).unwrap();

        assert_eq!(serialized, sample());
    }

    #[test]
    fn deserializes_correctly() {
        let deserialized: Beatmap = serde_json::from_str(&sample()).unwrap();

        assert_eq!(deserialized, manual_recreation());
    }
}
