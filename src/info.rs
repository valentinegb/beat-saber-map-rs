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
}

impl Default for Info {
    fn default() -> Self {
        Self {
            version: "4.0.0".to_string(),
            song: Default::default(),
            audio: Default::default(),
            song_preview_filename: "song.ogg".into(),
            cover_image_filename: "cover.png".into(),
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
            },
        );
    }
}
