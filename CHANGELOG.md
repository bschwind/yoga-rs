# Changelog

## 0.5.0
Released: 17th Jan 2025

- Upgrade to Yoga  [v3.2.1](https://github.com/facebook/yoga/commit/042f5013152eb81c1552dec945b88f7b95ca350f) (committed on: 13th Dec 2024)
- Added `Absolute` variant to `PositionType` enum
- Added `Contents` variant to `Display` enum
- Support percentage gaps
- Support `BoxSizing`
- Updated dependencies:
  - `ordered_float` from `3.x` to `4.x`
  - `bindgen` from `0.63.x` to `0.71.x`

## 0.4.0
Released: Dec 20th 2022

- Upgrade to Yoga commit [ba27f9d1ecfa7518019845b84b035d3d4a2a6658](https://github.com/facebook/yoga/commit/ba27f9d1ecfa7518019845b84b035d3d4a2a6658) (committed on: 16th Dec 2022)
- Added `Static` variant to `PositionType` enum
- Added bindings for gap styles: `Gutter` enum and `Node.set_gap()` and `Node.get_gap()` functions.
- Updated dependencies:
  - `ordered_float` from `1.0.1` to `3.4.0`
  - `cc` from `1.0.17` to `1.0.78`
  - `bindgen` from `0.37.0` to `0.63.0`
  - `serde` and `serde_derive` from `1.0.27` to `1.0.151`


## 0.3.1
Released: Jan 16th 2019

- Add `Node.mark_dirty()`
- Update dependencies:
  - `ordered_float` from `0.5.0` to `1.0.1`


## 0.3.0
Released: 16 Oct 2018

- Initial tagged version