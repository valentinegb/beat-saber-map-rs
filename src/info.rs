//! Module related to `Info.dat` map file.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Describes basic metadata about the song and points to map's other files.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html) for language-agnostic documentation.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Info {
    /// Should be "4.0.0", that's the currently supported schema version.
    pub version: String,
    /// See [`Song`].
    pub song: Song,
    /// See [`Audio`].
    pub audio: Audio,
    /// Audio file used for preview.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#audio-filename-s) for language-agnostic documentation.
    pub song_preview_filename: PathBuf,
    /// Cover image that displays alongside song metadata in selection menu.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#cover-image-filename) for language-agnostic documentation.
    pub cover_image_filename: PathBuf,
    /// Surrounding world that a player is within when playing a map.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#environments) for language-agnostic documentation.
    pub environment_names: Vec<String>,
    /// Color palettes used across in-game objects.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    pub color_schemes: Vec<ColorScheme>,
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
        }
    }
}

/// Describes basic metadata about the song.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-metadata) for language-agnostic documentation.
#[derive(Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Song {
    /// Title of map song.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-title) for language-agnostic documentation.
    pub title: String,
    /// Subtitle of map song, which may indicate any additional collaborators or alternative arrangements.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-subtitle) for language-agnostic documentation.
    #[serde(rename = "subTitle")]
    pub subtitle: String,
    /// Artist(s) of map's song.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-author) for language-agnostic documentation.
    pub author: String,
}

/// Audio metadata.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#audio-metadata) for language-agnostic documentation.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Audio {
    /// Audio file associated with map.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#audio-filename-s) for language-agnostic documentation.
    pub song_filename: PathBuf,
    /// Value (in seconds) which caches length of audio file.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-duration) for language-agnostic documentation.
    pub song_duration: f64,
    /// Audio metadata file associated with map.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#related-files) for language-agnostic documentation.
    pub audio_data_filename: PathBuf,
    /// Value which dictates how grid will align with audio file.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#bpm) for language-agnostic documentation.
    pub bpm: f64,
    /// Value which controls overall loudness of audio file.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#lufs-data-integrated) for language-agnostic documentation.
    pub lufs: f64,
    /// Time (in seconds) of song to start preview at.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-preview) for language-agnostic documentation.
    pub preview_start_time: f64,
    /// Duration (in seconds) of preview.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#song-preview) for language-agnostic documentation.
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

/// Color palette used across in-game objects.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ColorScheme {
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    pub use_override: bool,
    /// Player-facing name of color scheme.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    pub color_scheme_name: String,
    /// Color of left saber.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub saber_a_color: u32,
    /// Color of right saber.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub saber_b_color: u32,
    /// Color of wall obstacles.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub obstacles_color: u32,
    /// One of two environment colors.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub environment_color_0: u32,
    /// One of two environment colors.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub environment_color_1: u32,
    /// Boosted variant of one of two environment colors.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub environment_color_0_boost: u32,
    /// Boosted variant of one of two environment colors.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/info.html#color-schemes) for language-agnostic documentation.
    #[serde(with = "super::hex")]
    pub environment_color_1_boost: u32,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn deserializes_correctly() {
        let info: Info =
            serde_json::from_str(&fs::read_to_string("sample/Info.dat").unwrap()).unwrap();

        assert_eq!(
            info,
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
            },
        );
    }
}
