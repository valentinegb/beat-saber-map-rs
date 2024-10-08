# `beat_saber_map`

[![Crates.io Version](https://img.shields.io/crates/v/beat_saber_map)](https://crates.io/crates/beat_saber_map)
[![docs.rs](https://img.shields.io/docsrs/beat_saber_map)](https://docs.rs/beat_saber_map)
[![Rust](https://github.com/valentinegb/beat-saber-map-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/valentinegb/beat-saber-map-rs/actions/workflows/rust.yml)

Read and write Beat Saber maps with Rust.

```rs
use beat_saber_map::BeatSaberMap;

let map = BeatSaberMap::from_dir("sample").unwrap();

assert_eq!(map.info.version, "4.0.0");
assert_eq!(map.info.song.title, "Magic");
assert_eq!(map.info.environment_names.first().unwrap(), "WeaveEnvironment");
```

Documentation can be found on [docs.rs](https://docs.rs/beat_saber_map).

## Roadmap

- [ ] Create structures for map files
  - [x] [`Info`](https://docs.rs/beat_saber_map/latest/beat_saber_map/info/struct.Info.html)
  - [x] [`Audio`](https://docs.rs/beat_saber_map/latest/beat_saber_map/audio/struct.Audio.html)
  - [x] [`Beatmap`](https://docs.rs/beat_saber_map/latest/beat_saber_map/beatmap/struct.Beatmap.html)
  - [ ] `Lightshow`
- [ ] Create utility methods for map file structures and [`BeatSaberMap`](https://docs.rs/beat_saber_map/latest/beat_saber_map/struct.BeatSaberMap.html)
  - [x] [`BeatSaberMap::from_dir()`](https://docs.rs/beat_saber_map/latest/beat_saber_map/struct.BeatSaberMap.html#method.from_dir)
  - [x] [`Info::from_file()`](https://docs.rs/beat_saber_map/latest/beat_saber_map/info/struct.Info.html#method.from_file)
  - [x] [`Audio::from_file()`](https://docs.rs/beat_saber_map/latest/beat_saber_map/audio/struct.Audio.html#method.from_file)
  - [x] [`Beatmap::from_file()`](https://docs.rs/beat_saber_map/latest/beat_saber_map/beatmap/struct.Beatmap.html#method.from_file)
  - [ ] More...
