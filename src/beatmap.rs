//! Contains types related to beatmap files.
//!
//! See [`Beatmap`] to get started.

#![allow(deprecated)]

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Beats, Error};

/// Collections and associated metadata for all *interactable* beatmap items,
/// such as notes and obstacles.
#[doc = bsmg_wiki!("beatmap")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Beatmap {
    #[doc = version_doc!()]
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
    /// Instantiates a [`Beatmap`] from a beatmap file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}

/// The placement of an object.
#[doc = bsmg_wiki!("beatmap"#"color-notes")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Object {
    /// The specific point in time, as determined by the [BPM] of the song, when
    /// this object should reach the player.
    ///
    /// [BPM]: super::info::Audio::bpm
    #[serde(rename = "b")]
    pub beat: Beats,
    /// The degree of rotation relative to the player that this object should
    /// spawn from.
    ///
    /// This is typically reserved for [`Beatmap`]s using
    /// [`crate::info::Characteristic::ThreeSixtyDegree`] or
    /// [`crate::info::Characteristic::NinetyDegree`] characteristic.
    #[serde(rename = "r")]
    pub rotation_lane: i16,
    /// The index of corresponding data in `*_data` of [`Beatmap`].
    #[serde(rename = "i")]
    pub metadata_index: usize,
}

/// The attributes of a color note.
#[doc = bsmg_wiki!("beatmap"#"color-notes")]
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
    /// The angle offset. Applies a counter-clockwise rotational offset to a
    /// note's cut direction.
    #[doc = bsmg_wiki!("beatmap"#"color-notes-angle-offset")]
    #[serde(rename = "a")]
    pub angle_offset: i16,
}

/// The grid position of an obstacle.
#[doc = bsmg_wiki!("beatmap"#"color-notes")]
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

/// The horizontal row where an object should reside on the grid.
#[doc = bsmg_wiki!("beatmap"#"color-notes-line-index")]
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

/// The vertical column where an object should reside on the grid.
#[doc = bsmg_wiki!("beatmap"#"color-notes-line-layer")]
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

/// Which saber should be able to successfully cut a note.
#[doc = bsmg_wiki!("beatmap"#"color-notes-type")]
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

/// The direction the player should swing to successfully cut a note.
#[doc = bsmg_wiki!("beatmap"#"color-notes-cut-direction")]
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

/// The attributes of an obstacle.
#[doc = bsmg_wiki!("beatmap"#"obstacles")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ObstacleData {
    /// How long the obstacle extends for.
    #[doc = bsmg_wiki!("beatmap"#"obstacles-duration")]
    #[serde(rename = "d")]
    pub duration: Beats,
    /// See [`GridPosition`].
    #[serde(flatten)]
    pub grid_position: GridPosition,
    /// How many columns the obstacle should take up on the grid.
    #[doc = bsmg_wiki!("beatmap"#"obstacles-width")]
    #[serde(rename = "w")]
    pub width: i8,
    /// How many rows the obstacle should take up on the grid.
    ///
    /// The range of acceptable values runs from 1 to 5.
    #[doc = bsmg_wiki!("beatmap"#"obstacles-height")]
    #[serde(rename = "h")]
    pub height: i8,
}

/// The placement of an arc.
#[doc = bsmg_wiki!("beatmap"#"arcs")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Arc {
    /// The specific point in time, as determined by the [BPM] of the song, when
    /// the head of this arc should reach the player.
    ///
    /// [BPM]: super::info::Audio::bpm
    #[serde(rename = "hb")]
    pub head_beat: Beats,
    /// The specific point in time, as determined by the [BPM] of the song, when
    /// the tail of this arc should reach the player.
    ///
    /// [BPM]: super::info::Audio::bpm
    #[serde(rename = "tb")]
    pub tail_beat: Beats,
    /// The degree of rotation relative to the player that the head of this arc
    /// should spawn from.
    ///
    /// This is typically reserved for [`Beatmap`]s using
    /// [`crate::info::Characteristic::ThreeSixtyDegree`] or
    /// [`crate::info::Characteristic::NinetyDegree`] characteristic.
    #[serde(rename = "hr")]
    pub head_rotation_lane: i16,
    /// The degree of rotation relative to the player that the tail of this arc
    /// should spawn from.
    ///
    /// This is typically reserved for [`Beatmap`]s using
    /// [`crate::info::Characteristic::ThreeSixtyDegree`] or
    /// [`crate::info::Characteristic::NinetyDegree`] characteristic.
    #[serde(rename = "tr")]
    pub tail_rotation_lane: i16,
    /// The index of data corresponding to the head in [`Beatmap::color_notes_data`].
    #[serde(rename = "hi")]
    pub head_metadata_index: usize,
    /// The index of data corresponding to the tail in [`Beatmap::color_notes_data`].
    #[serde(rename = "ti")]
    pub tail_metadata_index: usize,
    /// The index of data corresponding to the arc in [`Beatmap::arcs_data`].
    #[serde(rename = "ai")]
    pub arc_metadata_index: usize,
}

/// The attributes of an [`Arc`].
#[doc = bsmg_wiki!("beatmap"#"arcs")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ArcData {
    /// The magnitude of the curve approaching the head.
    #[doc = bsmg_wiki!("beatmap"#"arcs-control-point-length-multiplier")]
    #[serde(rename = "m")]
    pub head_multiplier: f64,
    /// The magnitude of the curve approaching the tail.
    #[doc = bsmg_wiki!("beatmap"#"arcs-control-point-length-multiplier")]
    #[serde(rename = "tm")]
    pub tail_multiplier: f64,
    /// See [`MidAnchorMode`].
    #[serde(rename = "a")]
    pub mid_anchor_mode: MidAnchorMode,
}

/// How an [`Arc`] curves from its head/tail to the midpoint of the [`Arc`].
#[doc = bsmg_wiki!("beatmap"#"arcs-mid-anchor-mode")]
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

/// The placement of a chain.
#[doc = bsmg_wiki!("beatmap"#"chains")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Chain {
    /// The specific point in time, as determined by the [BPM] of the song, when
    /// the head of this chain should reach the player.
    ///
    /// [BPM]: super::info::Audio::bpm
    #[serde(rename = "hb")]
    pub head_beat: Beats,
    /// The specific point in time, as determined by the [BPM] of the song, when
    /// the tail of this chain should reach the player.
    ///
    /// [BPM]: super::info::Audio::bpm
    #[serde(rename = "tb")]
    pub tail_beat: Beats,
    /// The degree of rotation relative to the player that the head of this
    /// chain should spawn from.
    ///
    /// This is typically reserved for [`Beatmap`]s using
    /// [`crate::info::Characteristic::ThreeSixtyDegree`] or
    /// [`crate::info::Characteristic::NinetyDegree`] characteristic.
    #[serde(rename = "hr")]
    pub head_rotation_lane: i16,
    /// The degree of rotation relative to the player that the tail of this
    /// chain should spawn from.
    ///
    /// This is typically reserved for [`Beatmap`]s using
    /// [`crate::info::Characteristic::ThreeSixtyDegree`] or
    /// [`crate::info::Characteristic::NinetyDegree`] characteristic.
    #[serde(rename = "tr")]
    pub tail_rotation_lane: i16,
    /// The index of data corresponding to the head in
    /// [`Beatmap::color_notes_data`].
    #[serde(rename = "i")]
    pub head_metadata_index: usize,
    /// The index of data corresponding to the chain in
    /// [`Beatmap::chains_data`].
    #[serde(rename = "ci")]
    pub chain_metadata_index: usize,
}

/// The attributes of a [`Chain`].
#[doc = bsmg_wiki!("beatmap"#"chains")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ChainData {
    /// See [`LineIndex`].
    #[serde(rename = "tx")]
    pub tail_line_index: LineIndex,
    /// See [`LineLayer`].
    #[serde(rename = "ty")]
    pub tail_line_layer: LineLayer,
    /// The number of segments in the [`Chain`].
    ///
    /// The head counts as a segment.
    #[doc = bsmg_wiki!("beatmap"#"chains-slice-count")]
    #[serde(rename = "c")]
    pub slice_count: u8,
    /// The proportion of how much of the path from `(x, y)` to `(tx, ty)` is
    /// used by the [`Chain`].
    ///
    /// This does not alter the shape of the path.
    #[doc = bsmg_wiki!("beatmap"#"chains-squish-factor")]
    #[serde(rename = "s")]
    pub squish_factor: f64,
}

/// The placement of a spawn rotation.
#[doc = bsmg_wiki!("beatmap"#"spawn-rotations")]
#[deprecated = "use `beatmap::Object::rotation_lane` instead"]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SpawnRotation {
    /// The specific point in time, as determined by the [BPM] of the song, when
    /// this event should produce its effect.
    ///
    /// [BPM]: super::info::Audio::bpm
    #[serde(rename = "b")]
    pub beat: Beats,
    /// The index of corresponding data in [`Beatmap::spawn_rotations_data`].
    #[serde(rename = "i")]
    pub index: usize,
}

/// The attributes of [`SpawnRotation`].
#[doc = bsmg_wiki!("beatmap"#"spawn-rotations")]
#[deprecated = "use `beatmap::Object::rotation_lane` instead"]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SpawnRotationData {
    /// See [`ExecutionTime`].
    #[serde(rename = "t")]
    pub execution_time: ExecutionTime,
    /// The magnitude and direction of the lane rotation.
    #[doc = bsmg_wiki!("beatmap"#"spawn-rotations-magnitude")]
    #[serde(rename = "r")]
    pub magnitude: f64,
}

/// When a [`SpawnRotation`] should be applied to interactable objects placed on
/// the same beat as this event.
#[doc = bsmg_wiki!("beatmap"#"spawn-rotations-execution-time")]
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
