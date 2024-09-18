# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/ansg191/pandascore/compare/v0.5.0...v0.5.1) - 2024-09-18

### Added

- add more team endpoints

## [0.5.0](https://github.com/ansg191/pandascore/compare/v0.4.0...v0.5.0) - 2024-09-18

### Added

- add `PaginatedEndpoint` trait
- convert `ListMatches` to multi list endpoint
- [**breaking**] refactor `match` to `matches`

## [0.4.0](https://github.com/ansg191/pandascore/compare/v0.3.3...v0.4.0) - 2024-09-18

### Added

- add `non_exhaustive` attribute to all models
- add `current_team` field to `Player`

### Fixed

- make all `inner` fields `pub`

### Other

- remove debug `eprintln!`

## [0.3.3](https://github.com/ansg191/pandascore/compare/v0.3.2...v0.3.3) - 2024-09-14

### Fixed

- fix Winner parsing bug caused by mixed fields

## [0.3.2](https://github.com/ansg191/pandascore/compare/v0.3.1...v0.3.2) - 2024-09-13

### Fixed

- add underscore to `KEY_REGEX`
- pagination `Links` header parsing bug

## [0.3.1](https://github.com/ansg191/pandascore/compare/v0.3.0...v0.3.1) - 2024-09-12

### Other

- move transport to subtrait
- *(deps)* bump EmbarkStudios/cargo-deny-action from 1 to 2
- ignore dependabot commits via commitlint cfg

## [0.3.0](https://github.com/ansg191/pandascore/compare/v0.2.0...v0.3.0) - 2024-09-08

### Added

- [**breaking**] switch back to `bon`

### Other

- *(deps)* add `cargo-deny`

## [0.2.0](https://github.com/ansg191/pandascore/compare/v0.1.1...v0.2.0) - 2024-09-02

### Added
- add MSRV of 1.75.0

### Other
- add MSRV test
- switch from `LazyLock` to `OnceLock`
- Merge pull request [#13](https://github.com/ansg191/pandascore/pull/13) from ansg191/typed-builder
- switch to `typed_builder`

## [0.1.1](https://github.com/ansg191/pandascore/compare/v0.1.0...v0.1.1) - 2024-09-02

### Fixed
- *(clippy)* make clippy stricter

### Other
- add various documentation
- remove `unused_crate_dependencies` lint
- release

## [0.1.0](https://github.com/ansg191/pandascore/releases/tag/v0.1.0) - 2024-09-02

### Added
- *(all)* add tournament endpoints
- *(rl)* add Rocket League endpoints
- *(all)* add `Series` endpoints
- *(lol)* add `ListTeams` endpoint
- *(lol)* add `ListTournament` endpoint
- *(lol)* add `GetSpell` endpoint
- *(lol)* add `ListSpells` endpoint
- *(lol)* add `ListSeries` endpoint
- *(lol)* add `ListPlayers` endpoint
- *(lol)* add `ListMatches` endpoint
- *(lol)* add `ListLeagues` endpoint
- *(lol)* add `GetItem` endpoint
- *(lol)* add `ListItems` endpoint

### Other
- add `release-plz`
- switch to bon
- make all endpoints use macros
- *(lol)* convert to `multi_game_endpoints`
- *(lol)* convert lol to use `game_endpoints`
- update LoL features
- *(lol)* update LoL matches feature
- *(lol)* update LoL league feature
- update features docs
- Remove development files
- add commitlint
- add pre-commit
- Update LoL Champions status in docs
- Add `no_run` to lib docs example code
- Disable `fail-fast` for test CI
- Fix linter problems
- Add CI
- Initial Commit
