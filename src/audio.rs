//! Contains types related to `BPMInfo.bat` files.
//!
//! See [`Audio`] to get started.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Beats, Error};

/// Information regarding how an audio file should be processed.
#[doc = bsmg_wiki!("audio")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Audio {
    #[doc = version_doc!()]
    pub version: String,
    /// Used for verifying internal relationships and leaderboard integrity.
    #[doc = bsmg_wiki!(#"checksums")]
    pub song_checksum: String,
    /// The duration of the audio file in samples.
    #[doc = bsmg_wiki!("audio"#"sample-count")]
    pub song_sample_count: u32,
    /// The cached quality level of the audio file.
    #[doc = bsmg_wiki!("audio"#"song-frequency")]
    pub song_frequency: u32,
    /// See [`BpmData`].
    pub bpm_data: Vec<BpmData>,
    /// See [`LufsData`].
    pub lufs_data: Vec<LufsData>,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            version: "4.0.0".to_string(),
            song_checksum: Default::default(),
            song_sample_count: Default::default(),
            song_frequency: Default::default(),
            bpm_data: Default::default(),
            lufs_data: Default::default(),
        }
    }
}

impl Audio {
    /// Instantiates an [`Audio`] from an audio file, typically named
    /// `BPMInfo.dat`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}

/// Regions in an [`Audio`] to alter the BPM of.
#[doc = bsmg_wiki!("audio"#"bpm-regions")]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct BpmData {
    /// The starting sample index.
    #[serde(rename = "si")]
    pub start_index: usize,
    /// The ending sample index.
    #[serde(rename = "ei")]
    pub end_index: usize,
    /// The starting beat.
    #[serde(rename = "sb")]
    pub start_beat: Beats,
    /// The ending beat.
    #[serde(rename = "eb")]
    pub end_beat: Beats,
}

/// Normalization to apply to the loudness of an [`Audio`] within the specified
/// region.
#[doc = bsmg_wiki!("audio"#"lufs-data")]
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LufsData {
    /// The starting sample index.
    #[serde(rename = "si")]
    pub start_index: usize,
    /// The ending sample index.
    #[serde(rename = "ei")]
    pub end_index: usize,
    /// The loudness.
    #[serde(rename = "l")]
    pub loudness: usize,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn sample() -> String {
        fs::read_to_string("sample/BPMInfo.dat").unwrap()
    }

    fn manual_recreation() -> Audio {
        Audio {
            version: "4.0.0".to_string(),
            song_checksum: "".to_string(),
            song_sample_count: 1149214,
            song_frequency: 44100,
            bpm_data: vec![BpmData {
                start_index: 0,
                end_index: 1149214,
                start_beat: 0.0,
                end_beat: 26.0,
            }],
            lufs_data: vec![LufsData {
                start_index: 0,
                end_index: 1149214,
                loudness: 0,
            }],
        }
    }

    #[test]
    fn serializes_correctly() {
        let serialized = serde_json::to_string_pretty(&manual_recreation()).unwrap();

        assert_eq!(serialized, sample());
    }

    #[test]
    fn deserializes_correctly() {
        let deserialized: Audio = serde_json::from_str(&sample()).unwrap();

        assert_eq!(deserialized, manual_recreation());
    }
}
