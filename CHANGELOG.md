# Changelog
All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.13.0] - 2019-07-04
### Changed
- Updated to API version **5.101**.

## [0.12.0] - 2019-05-11
### Changed
- `APIClient::new` now accepts `token: impl Into<String>` instead of `token: String`.
- Updated to API version **5.95**.

## [0.11.0] - 2019-02-21
### Changed
- Added `extra` field to `APIError` ([#4](https://github.com/u32i64/rvk/pull/4)).
- `impl From<&str>` instead of `impl From<&'static str>` for `APIError`.
- Some tweaks to docs.

### Added
- New methods. See [`9ca852c`](https://github.com/u32i64/rvk/commit/9ca852cbd9154a6a9374fe727bcd06c16dfe7111).

## [0.10.0] - 2019-02-17
> **yanked**, changes reverted.
### Added
- `objects::message::Message`: add `pub to_id: Option<Integer>` and `pub state: Option<String>`.
### Changed
- `objects::message::Message`'s `peer_id` field is now an `Option<Integer>` (was `Integer`).

## [0.9.1] - 2019-02-12
### Added
- `#[derive(Debug)]` for `APIClient`

## [0.9.0] - 2018-12-07
### Changed
- Rust 2018 :tada:

## [0.8.0] - 2018-11-24
### Changed
- Updated to API version **5.92**.

## [0.7.0] - 2018-10-27
### Added
- `messages.getRecentCalls` method.

### Changed
- Improved performance using `lazy_static!`.
- Updated to API version **5.87**.

## [0.6.0] - 2018-09-14
### Changed
- Updated to API version **5.85**. See the [VK API Changelog](https://vk.com/dev/versions).

## [0.5.0] - 2018-07-19
### Added
- Objects! See [`rvk::objects (v0.5)`](https://docs.rs/rvk/0.5/rvk/objects/index.html).

## [0.4.0] - 2018-06-26
### Changed
- Type [`rvk::Params (v0.3.1)`](https://docs.rs/rvk/0.3.1/rvk/type.Params.html) was moved to [`rvk::api::Params (v0.4.0)`](https://docs.rs/rvk/0.4.0/rvk/api/type.Params.html), but it is now re-exported as `rvk::Params` too.

## [0.3.0] - 2018-06-19
### Changed
- This renames/adds some methods for VK API version **5.80** according to the [API Changelog](https://vk.com/dev/versions).

## [0.2.0] - 2018-05-31
### Changed
- API is now accessed synchronously
- Improved error handling

## [0.1.0] - 2018-05-26
### First release


[0.12.0]: https://github.com/u32i64/rvk/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/u32i64/rvk/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/u32i64/rvk/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/u32i64/rvk/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/u32i64/rvk/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/u32i64/rvk/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/u32i64/rvk/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/u32i64/rvk/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/u32i64/rvk/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/u32i64/rvk/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/u32i64/rvk/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/u32i64/rvk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/u32i64/rvk/releases/tag/v0.1.0
