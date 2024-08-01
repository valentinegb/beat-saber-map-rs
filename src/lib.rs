//! Read and write Beat Saber maps with Rust.
//!
//! Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for language-agnostic documentation.

#![warn(missing_docs)]

mod hex;
pub mod info;

pub use self::info::Info;

/// Structural representation of a Beat Saber map folder.
///
/// Refer to the [BSMG Wiki](https://bsmg.wiki/mapping/map-format.html) for language-agnostic documentation.
#[derive(Debug, PartialEq, Default)]
pub struct BeatSaberMap {
    /// `Info.dat` file.
    ///
    /// See [`Info`].
    pub info: Info,
}
