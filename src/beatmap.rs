//! Module related to beatmap files.
#![allow(deprecated)]

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Beat, Error};

/// Defines collections and associated metadata for all *interactable* beatmap
/// items, such as notes and obstacles.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Beatmap {
    /// Should be "4.0.0", that's the currently supported schema version.
    pub version: String,
    /// See [`Object`].
    pub color_notes: Vec<Object>,
    /// See [`ColorNoteData`].
    pub color_notes_data: Vec<ColorNoteData>,
    /// See [`Object`].
    pub bomb_notes: Vec<Object>,
    /// See [`GridPosition`].
    pub bomb_notes_data: Vec<GridPosition>,
    /// See [`Object`].
    pub obstacles: Vec<Object>,
    /// See [`ObstacleData`].
    pub obstacles_data: Vec<ObstacleData>,
    /// See [`Arc`].
    pub arcs: Vec<Arc>,
    /// See [`ArcData`].
    pub arcs_data: Vec<ArcData>,
    /// See [`Chain`].
    pub chains: Vec<Chain>,
    /// See [`ChainData`].
    pub chains_data: Vec<ChainData>,
    /// See [`SpawnRotation`].
    #[deprecated = "use `beatmap::Object::rotation_lane` instead"]
    pub spawn_rotations: Vec<SpawnRotation>,
    /// See [`SpawnRotationData`].
    #[deprecated = "use `beatmap::Object::rotation_lane` instead"]
    pub spawn_rotations_data: Vec<SpawnRotationData>,
}

impl Default for Beatmap {
    fn default() -> Self {
        Self {
            version: "4.0.0".to_string(),
            color_notes: Default::default(),
            color_notes_data: Default::default(),
            bomb_notes: Default::default(),
            bomb_notes_data: Default::default(),
            obstacles: Default::default(),
            obstacles_data: Default::default(),
            arcs: Default::default(),
            arcs_data: Default::default(),
            chains: Default::default(),
            chains_data: Default::default(),
            spawn_rotations: Default::default(),
            spawn_rotations_data: Default::default(),
        }
    }
}

impl Beatmap {
    /// Instatiates a [`Beatmap`] from a beatmap file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}

/// Placement of object.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Object {
    /// Specific point in time, as determined by [BPM](super::info::Audio::bpm)
    /// of song, when this object should reach player.
    #[serde(rename = "b")]
    pub beat: Beat,
    /// Degree of rotation relative to player that this object should spawn
    /// from. This is typically reserved for beatmaps using `360Degree` or
    /// `90Degree` characteristic.
    #[serde(rename = "r")]
    pub rotation_lane: i16,
    /// Index of corresponding data in `*_data` of [`Beatmap`].
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
    /// See [`GridPosition`].
    #[serde(flatten)]
    pub grid_position: GridPosition,
    /// See [`Color`].
    #[serde(rename = "c")]
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

/// Grid position of obstacle.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#color-notes)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct GridPosition {
    /// See [`LineIndex`].
    #[serde(rename = "x")]
    pub line_index: LineIndex,
    /// See [`LineLayer`].
    #[serde(rename = "y")]
    pub line_layer: LineLayer,
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

/// Attributes of obstacle.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#obstacles)
/// for language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ObstacleData {
    /// How long obstacle extends for (in beats).
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#obstacles-duration)
    /// for language-agnostic documentation.
    #[serde(rename = "d")]
    pub duration: Beat,
    /// See [`GridPosition`].
    #[serde(flatten)]
    pub grid_position: GridPosition,
    /// How many columns obstacle should take up on grid.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#obstacles-width)
    /// for language-agnostic documentation.
    #[serde(rename = "w")]
    pub width: i8,
    /// How many rows obstacle should take up on grid. Range of acceptable
    /// values runs from 1 to 5.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#obstacles-height)
    /// for language-agnostic documentation.
    #[serde(rename = "h")]
    pub height: i8,
}

/// Placement of arc.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#arcs) for
/// language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Arc {
    /// Specific point in time, as determined by [BPM](super::info::Audio::bpm)
    /// of song, when head of this arc should reach player.
    #[serde(rename = "hb")]
    pub head_beat: Beat,
    /// Specific point in time, as determined by [BPM](super::info::Audio::bpm)
    /// of song, when tail of this arc should reach player.
    #[serde(rename = "tb")]
    pub tail_beat: Beat,
    /// Degree of rotation relative to player that head of this arc should spawn
    /// from. This is typically reserved for beatmaps using `360Degree` or
    /// `90Degree` characteristic.
    #[serde(rename = "hr")]
    pub head_rotation_lane: i16,
    /// Degree of rotation relative to player that tail of this arc should spawn
    /// from. This is typically reserved for beatmaps using `360Degree` or
    /// `90Degree` characteristic.
    #[serde(rename = "tr")]
    pub tail_rotation_lane: i16,
    /// Index of data corresponding to head in [`Beatmap::color_notes_data`].
    #[serde(rename = "hi")]
    pub head_metadata_index: usize,
    /// Index of data corresponding to tail in [`Beatmap::color_notes_data`].
    #[serde(rename = "ti")]
    pub tail_metadata_index: usize,
    /// Index of data corresponding to arc in [`Beatmap::arcs_data`].
    #[serde(rename = "ai")]
    pub arc_metadata_index: usize,
}

/// Attributes of arc.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#arcs) for
/// language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ArcData {
    /// Magnitude of curve approaching the head.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#arcs-control-point-length-multiplier)
    /// for language-agnostic documentation.
    #[serde(rename = "m")]
    pub head_multiplier: f64,
    /// Magnitude of curve approaching the tail.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#arcs-control-point-length-multiplier)
    /// for language-agnostic documentation.
    #[serde(rename = "tm")]
    pub tail_multiplier: f64,
    /// See [`MidAnchorMode`].
    #[serde(rename = "a")]
    pub mid_anchor_mode: MidAnchorMode,
}

/// How arc curves from head/tail to midpoint of arc.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#arcs-mid-anchor-mode)
/// for language-agnostic documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum MidAnchorMode {
    #[default]
    Straight,
    Clockwise,
    CounterClockwise,
}

impl TryFrom<u8> for MidAnchorMode {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Straight),
            1 => Ok(Self::Clockwise),
            2 => Ok(Self::CounterClockwise),
            other => Err(Error::MidAnchorModeTryFromU8(other)),
        }
    }
}

impl Into<u8> for MidAnchorMode {
    fn into(self) -> u8 {
        self as u8
    }
}

/// Placement of chain.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#chains) for
/// language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Chain {
    /// Specific point in time, as determined by [BPM](super::info::Audio::bpm)
    /// of song, when head of this chain should reach player.
    #[serde(rename = "hb")]
    pub head_beat: Beat,
    /// Specific point in time, as determined by [BPM](super::info::Audio::bpm)
    /// of song, when tail of this chain should reach player.
    #[serde(rename = "tb")]
    pub tail_beat: Beat,
    /// Degree of rotation relative to player that head of this chain should
    /// spawn from. This is typically reserved for beatmaps using `360Degree` or
    /// `90Degree` characteristic.
    #[serde(rename = "hr")]
    pub head_rotation_lane: i16,
    /// Degree of rotation relative to player that tail of this chain should
    /// spawn from. This is typically reserved for beatmaps using `360Degree` or
    /// `90Degree` characteristic.
    #[serde(rename = "tr")]
    pub tail_rotation_lane: i16,
    /// Index of data corresponding to head in [`Beatmap::color_notes_data`].
    #[serde(rename = "i")]
    pub head_metadata_index: usize,
    /// Index of data corresponding to chain in [`Beatmap::chains_data`].
    #[serde(rename = "ci")]
    pub chain_metadata_index: usize,
}

/// Attributes of chain.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#chains) for
/// language-agnostic documentation.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ChainData {
    /// See [`LineIndex`].
    #[serde(rename = "tx")]
    pub tail_line_index: LineIndex,
    /// See [`LineLayer`].
    #[serde(rename = "ty")]
    pub tail_line_layer: LineLayer,
    /// Number of segments in chain. Head counts as segment.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#chains-slice-count)
    /// for language-agnostic documentation.
    #[serde(rename = "c")]
    pub slice_count: u8,
    /// Proportion of how much of path from `(x, y)` to `(tx, ty)` is used by
    /// chain. This does not alter shape of path.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#chains-squish-factor)
    /// for language-agnostic documentation.
    #[serde(rename = "s")]
    pub squish_factor: f64,
}

/// Placement of spawn rotation.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#spawn-rotations)
/// for language-agnostic documentation.
#[deprecated = "use `beatmap::Object::rotation_lane` instead"]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SpawnRotation {
    /// Specific point in time, as determined by [BPM](super::info::Audio::bpm)
    /// of song, when this event should produce its effect.
    #[serde(rename = "b")]
    pub beat: Beat,
    /// Index of corresponding data in [`Beatmap::spawn_rotations_data`].
    #[serde(rename = "i")]
    pub index: usize,
}

/// Attributes of spawn rotation.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#spawn-rotations)
/// for language-agnostic documentation.
#[deprecated = "use `beatmap::Object::rotation_lane` instead"]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SpawnRotationData {
    /// See [`ExecutionTime`].
    #[serde(rename = "t")]
    pub execution_time: ExecutionTime,
    /// Magnitude and direction of lane rotation.
    ///
    /// Refer to the
    /// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#spawn-rotations-magnitude)
    /// for language-agnostic documentation.
    #[serde(rename = "r")]
    pub magnitude: f64,
}

/// When lane rotation will be applied to interactable objects placed on same
/// beat as this event.
///
/// Refer to the
/// [BSMG Wiki](https://bsmg.wiki/mapping/map-format/beatmap.html#spawn-rotations-execution-time)
/// for language-agnostic documentation.
#[allow(missing_docs)]
#[deprecated = "`beatmap::SpawnRotationData` is deprecated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ExecutionTime {
    #[default]
    Early,
    Late,
}

impl TryFrom<u8> for ExecutionTime {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Early),
            1 => Ok(Self::Late),
            other => Err(Error::ExecutionTimeTryFromU8(other)),
        }
    }
}

impl Into<u8> for ExecutionTime {
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
            color_notes: vec![Object {
                beat: 10.0,
                rotation_lane: 0,
                metadata_index: 0,
            }],
            color_notes_data: vec![
                ColorNoteData {
                    grid_position: GridPosition {
                        line_index: LineIndex::Left,
                        line_layer: LineLayer::Bottom,
                    },
                    color: Color::LeftSaber,
                    cut_direction: CutDirection::Down,
                    angle_offset: 0,
                },
                ColorNoteData {
                    grid_position: GridPosition {
                        line_index: LineIndex::Right,
                        line_layer: LineLayer::Top,
                    },
                    color: Color::LeftSaber,
                    cut_direction: CutDirection::Up,
                    angle_offset: 0,
                },
            ],
            bomb_notes: vec![Object {
                beat: 10.0,
                rotation_lane: 0,
                metadata_index: 0,
            }],
            bomb_notes_data: vec![GridPosition {
                line_index: LineIndex::Left,
                line_layer: LineLayer::Bottom,
            }],
            obstacles: vec![Object {
                beat: 10.0,
                rotation_lane: 0,
                metadata_index: 0,
            }],
            obstacles_data: vec![ObstacleData {
                duration: 5.0,
                grid_position: GridPosition {
                    line_index: LineIndex::Left,
                    line_layer: LineLayer::Bottom,
                },
                width: 1,
                height: 5,
            }],
            arcs: vec![Arc {
                head_beat: 10.0,
                tail_beat: 15.0,
                head_rotation_lane: 0,
                tail_rotation_lane: 0,
                head_metadata_index: 0,
                tail_metadata_index: 1,
                arc_metadata_index: 0,
            }],
            arcs_data: vec![ArcData {
                head_multiplier: 1.0,
                tail_multiplier: 1.0,
                mid_anchor_mode: MidAnchorMode::Straight,
            }],
            chains: vec![Chain {
                head_beat: 10.0,
                tail_beat: 15.0,
                head_rotation_lane: 0,
                tail_rotation_lane: 0,
                head_metadata_index: 0,
                chain_metadata_index: 0,
            }],
            chains_data: vec![ChainData {
                tail_line_index: LineIndex::Right,
                tail_line_layer: LineLayer::Top,
                slice_count: 3,
                squish_factor: 0.5,
            }],
            spawn_rotations: vec![
                SpawnRotation {
                    beat: 10.0,
                    index: 0,
                },
                SpawnRotation {
                    beat: 15.0,
                    index: 1,
                },
            ],
            spawn_rotations_data: vec![
                SpawnRotationData {
                    execution_time: ExecutionTime::Early,
                    magnitude: 15.0,
                },
                SpawnRotationData {
                    execution_time: ExecutionTime::Late,
                    magnitude: 15.0,
                },
            ],
        }
    }

    #[test]
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
