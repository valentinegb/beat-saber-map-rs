//! Contains types related to `Info.bat` files.
//!
//! See [`Info`] to get started.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{Beats, Error};

/// Describes basic metadata about the song and points to a map's other files.
#[doc = bsmg_wiki!("info")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Info {
    #[doc = version_doc!()]
    pub version: String,
    /// See [`Song`].
    pub song: Song,
    /// See [`Audio`].
    pub audio: Audio,
    /// The audio file used for the in-game preview.
    #[doc = bsmg_wiki!("info"#"audio-filename-s")]
    pub song_preview_filename: PathBuf,
    /// The cover image that displays alongside the song metadata in the
    /// selection menu.
    #[doc = bsmg_wiki!("info"#"cover-image-filename")]
    pub cover_image_filename: PathBuf,
    /// The surrounding world that a player is within when playing the map.
    #[doc = bsmg_wiki!("info"#"environments")]
    pub environment_names: Vec<String>,
    /// The color palettes used across in-game objects.
    #[doc = bsmg_wiki!("info"#"color-schemes")]
    pub color_schemes: Vec<ColorScheme>,
    /// See [`DifficultyBeatmap`].
    pub difficulty_beatmaps: Vec<DifficultyBeatmap>,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            version: "4.0.0".to_string(),
            song: Default::default(),
            audio: Default::default(),
            song_preview_filename: "song.ogg".into(),
            cover_image_filename: "cover.png".into(),
            environment_names: Default::default(),
            color_schemes: Default::default(),
            difficulty_beatmaps: Default::default(),
        }
    }
}

impl Info {
    /// Instantiates an [`Info`] from an info file, typically named `Info.dat`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}

/// Describes basic metadata about the song.
#[doc = bsmg_wiki!("info"#"song-metadata")]
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Song {
    /// The title of the map's song.
    #[doc = bsmg_wiki!("info"#"song-title")]
    pub title: String,
    /// The subtitle of the map's song, which may indicate any additional
    /// collaborators or alternative arrangements.
    #[doc = bsmg_wiki!("info"#"song-subtitle")]
    #[serde(rename = "subTitle")]
    pub subtitle: String,
    /// The artist(s) of the map's song.
    #[doc = bsmg_wiki!("info"#"song-author")]
    pub author: String,
}

/// Audio metadata.
#[doc = bsmg_wiki!("info"#"audio-metadata")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Audio {
    /// The audio file associated with the map.
    #[doc = bsmg_wiki!("info"#"audio-filename-s")]
    pub song_filename: PathBuf,
    /// The cached length of the audio file (in seconds).
    #[doc = bsmg_wiki!("info"#"song-duration")]
    pub song_duration: f64,
    /// The audio metadata file associated with the map.
    #[doc = bsmg_wiki!("info"#"related-files")]
    pub audio_data_filename: PathBuf,
    /// How the grid will align with the audio file.
    #[doc = bsmg_wiki!("info"#"bpm")]
    pub bpm: Beats,
    /// The overall loudness of the audio file.
    #[doc = bsmg_wiki!("info"#"lufs-data-integrated")]
    pub lufs: f64,
    /// The time (in seconds) of the song to start the preview at.
    #[doc = bsmg_wiki!("info"#"song-preview")]
    pub preview_start_time: f64,
    /// The duration (in seconds) of the preview.
    #[doc = bsmg_wiki!("info"#"song-preview")]
    pub preview_duration: f64,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            song_filename: "song.ogg".into(),
            song_duration: Default::default(),
            audio_data_filename: "BPMInfo.dat".into(),
            bpm: Default::default(),
            lufs: Default::default(),
            preview_start_time: Default::default(),
            preview_duration: Default::default(),
        }
    }
}

/// A color palette used across in-game objects.
#[doc = bsmg_wiki!("info"#"color-schemes")]
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ColorScheme {
    #[doc = bsmg_wiki!("info"#"color-schemes")]
    pub use_override: bool,
    /// The player-facing name of the color scheme.
    pub color_scheme_name: String,
    /// The color of the left saber.
    #[serde(with = "super::hex")]
    pub saber_a_color: u32,
    /// The color of the right saber.
    #[serde(with = "super::hex")]
    pub saber_b_color: u32,
    /// The color of wall obstacles.
    #[serde(with = "super::hex")]
    pub obstacles_color: u32,
    /// The first environment color.
    #[serde(with = "super::hex")]
    pub environment_color_0: u32,
    /// The second environment color.
    #[serde(with = "super::hex")]
    pub environment_color_1: u32,
    /// Boosted variant of the first environment color.
    #[serde(with = "super::hex")]
    pub environment_color_0_boost: u32,
    /// Boosted variant of the second environment color.
    #[serde(with = "super::hex")]
    pub environment_color_1_boost: u32,
}

/// An individual level associated with a map, organized by its characteristic
/// and difficulty.
#[doc = bsmg_wiki!("info"#"beatmap-metadata")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct DifficultyBeatmap {
    /// See [`Characteristic`].
    pub characteristic: Characteristic,
    /// See [`Difficulty`].
    pub difficulty: Difficulty,
    /// See [`BeatmapAuthors`].
    pub beatmap_authors: BeatmapAuthors,
    /// The index of an environment from [`Info::environment_names`].
    #[doc = bsmg_wiki!("info"#"environments")]
    pub environment_name_idx: usize,
    /// The index of a color scheme from [`Info::color_schemes`].
    #[doc = bsmg_wiki!("info"#"color-schemes")]
    pub beatmap_color_scheme_idx: usize,
    /// The speed at which objects in the beatmap will move torwards the player.
    #[doc = bsmg_wiki!("info"#"note-jump-metadata")]
    pub note_jump_movement_speed: u32,
    /// The offset at which objects in the beatmap will move torwards the
    /// player.
    #[doc = bsmg_wiki!("info"#"note-jump-metadata")]
    pub note_jump_start_beat_offset: Beats,
    /// The level file for interactable objects associated with the map.
    #[doc = bsmg_wiki!("info"#"beatmap-filename")]
    pub beatmap_data_filename: PathBuf,
    /// The level file for non-interactable objects associated with the map.
    #[doc = bsmg_wiki!("info"#"beatmap-filename")]
    pub lightshow_data_filename: PathBuf,
}

/// Groups [`DifficultyBeatmap`]s into unique categories and applies specialized
/// behaviors to those affected [`DifficultyBeatmap`]s.
#[doc = bsmg_wiki!("info"#"characteristic")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum Characteristic {
    /// No special behavior.
    #[default]
    Standard,
    /// No special behavior.
    NoArrows,
    /// Disables the left (red) saber.
    OneSaber,
    /// Uses rotation behaviors.
    #[serde(rename = "360Degree")]
    ThreeSixtyDegree,
    /// Uses rotation behaviors.
    #[serde(rename = "90Degree")]
    NinetyDegree,
    /// No special behavior.
    Legacy,
}

/// A cosmetic label to indicate the overall difficulty of a
/// [`DifficultyBeatmap`], relative to its [`Characteristic`].
#[doc = bsmg_wiki!("info"#"difficulty")]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}

/// The designer(s) of a [`DifficultyBeatmap`], including any contributing
/// mappers and lighters.
#[doc = bsmg_wiki!("info"#"beatmap-authors")]
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct BeatmapAuthors {
    /// The map designer(s) of a [`DifficultyBeatmap`].
    pub mappers: Vec<String>,
    /// The light designer(s) of a [`DifficultyBeatmap`].
    pub lighters: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn sample() -> String {
        fs::read_to_string("sample/Info.dat").unwrap()
    }

    fn manual_recreation() -> Info {
        Info {
            version: "4.0.0".to_string(),
            song: Song {
                title: "Magic".to_string(),
                subtitle: "ft. Meredith Bull".to_string(),
                author: "Jaroslav Beck".to_string(),
            },
            audio: Audio {
                song_filename: "song.ogg".into(),
                song_duration: 202.0,
                audio_data_filename: "BPMInfo.dat".into(),
                bpm: 208.0,
                lufs: 0.0,
                preview_start_time: 0.0,
                preview_duration: 0.0,
            },
            song_preview_filename: "song.ogg".into(),
            cover_image_filename: "cover.png".into(),
            environment_names: vec![
                "WeaveEnvironment".to_string(),
                "GlassDesertEnvironment".to_string(),
            ],
            color_schemes: vec![ColorScheme {
                use_override: true,
                color_scheme_name: "Weave".to_string(),
                saber_a_color: 0xC81414FF,
                saber_b_color: 0x288ED2FF,
                obstacles_color: 0xFF3030FF,
                environment_color_0: 0xD91616FF,
                environment_color_1: 0x30ACFFFF,
                environment_color_0_boost: 0xD216D9FF,
                environment_color_1_boost: 0x00FFA5FF,
            }],
            difficulty_beatmaps: vec![
                DifficultyBeatmap {
                    characteristic: Characteristic::Standard,
                    difficulty: Difficulty::Easy,
                    beatmap_authors: BeatmapAuthors {
                        mappers: vec!["Freeek".to_string()],
                        lighters: vec!["Freeek".to_string()],
                    },
                    environment_name_idx: 0,
                    beatmap_color_scheme_idx: 0,
                    note_jump_movement_speed: 10,
                    note_jump_start_beat_offset: 0.0,
                    beatmap_data_filename: "Easy.dat".into(),
                    lightshow_data_filename: "Lightshow.dat".into(),
                },
                DifficultyBeatmap {
                    characteristic: Characteristic::Standard,
                    difficulty: Difficulty::Normal,
                    beatmap_authors: BeatmapAuthors {
                        mappers: vec!["Freeek".to_string()],
                        lighters: vec!["Freeek".to_string()],
                    },
                    environment_name_idx: 0,
                    beatmap_color_scheme_idx: 0,
                    note_jump_movement_speed: 10,
                    note_jump_start_beat_offset: 0.0,
                    beatmap_data_filename: "Normal.dat".into(),
                    lightshow_data_filename: "Lightshow.dat".into(),
                },
                DifficultyBeatmap {
                    characteristic: Characteristic::Standard,
                    difficulty: Difficulty::Hard,
                    beatmap_authors: BeatmapAuthors {
                        mappers: vec!["Freeek".to_string()],
                        lighters: vec!["Freeek".to_string()],
                    },
                    environment_name_idx: 0,
                    beatmap_color_scheme_idx: 0,
                    note_jump_movement_speed: 10,
                    note_jump_start_beat_offset: 0.0,
                    beatmap_data_filename: "Hard.dat".into(),
                    lightshow_data_filename: "Lightshow.dat".into(),
                },
                DifficultyBeatmap {
                    characteristic: Characteristic::Standard,
                    difficulty: Difficulty::Expert,
                    beatmap_authors: BeatmapAuthors {
                        mappers: vec!["Freeek".to_string()],
                        lighters: vec!["Freeek".to_string()],
                    },
                    environment_name_idx: 0,
                    beatmap_color_scheme_idx: 0,
                    note_jump_movement_speed: 16,
                    note_jump_start_beat_offset: 1.0,
                    beatmap_data_filename: "Expert.dat".into(),
                    lightshow_data_filename: "Lightshow.dat".into(),
                },
                DifficultyBeatmap {
                    characteristic: Characteristic::Standard,
                    difficulty: Difficulty::ExpertPlus,
                    beatmap_authors: BeatmapAuthors {
                        mappers: vec!["Freeek".to_string()],
                        lighters: vec!["Freeek".to_string()],
                    },
                    environment_name_idx: 0,
                    beatmap_color_scheme_idx: 0,
                    note_jump_movement_speed: 18,
                    note_jump_start_beat_offset: 0.5,
                    beatmap_data_filename: "ExpertPlus.dat".into(),
                    lightshow_data_filename: "LightshowPlus.dat".into(),
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
        let deserialized: Info = serde_json::from_str(&sample()).unwrap();

        assert_eq!(deserialized, manual_recreation());
    }
}
