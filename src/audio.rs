//! Module related to `BPMInfo.dat` map file.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::Error;

/// Information regarding how audio file should be processed.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/audio.html) for language-agnostic documentation.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Audio {
    /// Should be "4.0.0", that's the currently supported schema version.
    pub version: String,
    /// Used for verifying internal relationships and leaderboard integrity.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html#checksums) for language-agnostic documentation.
    pub song_checksum: String,
    /// Measures duration of audio file in samples.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/audio.html#sample-count) for language-agnostic documentation.
    pub song_sample_count: u32,
    /// Caches quality level of audio file.
    ///
    /// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/audio.html#song-frequency) for language-agnostic documentation.
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
    /// Instatiates an [`Audio`] from an audio file, typically named `BPMInfo.dat`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}

/// Alters BPM of specified region.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/audio.html#bpm-regions) for language-agnostic documentation.
#[derive(Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct BpmData {
    /// Start sample index.
    pub si: usize,
    /// End sample index.
    pub ei: usize,
    /// Start beat.
    pub sb: usize,
    /// End beat.
    pub eb: usize,
}

/// Applies normalization to loudness of audio file within specified region.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format/audio.html#lufs-data) for language-agnostic documentation.
#[derive(Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LufsData {
    /// Start sample index.
    pub si: usize,
    /// End sample index.
    pub ei: usize,
    /// Loudness.
    pub l: usize,
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
                si: 0,
                ei: 1149214,
                sb: 0,
                eb: 26,
            }],
            lufs_data: vec![LufsData {
                si: 0,
                ei: 1149214,
                l: 0,
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