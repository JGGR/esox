## [0.1.3] - Unreleased

### Added

- Add `csv::load` module
  - New methods wrap over the errors of deser/parse steps
- Add `get_x3_a()`, `get_x3_b()` to `RisultatoNISECI`
- Add `to_csv()` for:
  - `domain::niseci::{RisultatoNISECI, ValoriIntermediNISECI}`
  - `domain::hfbi::{RisultatoHFBI, ValoriIntermediHFBI}`
- Add `impl From<(f32, &AreaNISECI)>` to `domain::niseci::StatoEcologicoNISECI`
- Add `impl From<f32>` to `domain::hfbi::StatoEcologicoHFBI`
- Add `impl Default` to `domain::niseci::{ClassiEtaSpecieNISECI, ClassiEtaAlieniNISECI, InfoPopolazioniNISECI}`
- Add `meta::version()` to get crate version

### Changed

- Drop `once_cell` dependency by replacing `once_cell::sync::Lazy` with `std::sync::LazyLock`
  - Needs `rust 1.80`
- Make types related to full calc results `Deserialize`
- Make types related to full calc input `Serialize`, `Deserialize`
- Add `#[serde(deny_unknown_fields)]` to `Deserialize` types
  - Types with `Deserialize` in `csv::deser` modules did not receive this annotation since it would be a breaking change
- Moved templates data for tests from `tests::engines::{hfbi, niseci}::full` to `tests::test_utils`
- Moved constants for `StatoEcologicoNISECI` from `tests::engines::niseci::full` to `domain::niseci`
- Bump deps
- Turn off default features for `chrono`

## [0.1.2] - 2026-02-06

### Changed

- Refactor `RecordCsvCampionamentoNISECI` trait to have `peso` be `f32`
- Check and refuse `NaN` and `Inf` for `peso` for `RecordNISECI`, `RecordHFBI`
- Add `.github/dependabot.yml`
- Update copyright

## [0.1.1] - 2025-12-19

### Changed

- Make types related to full calc results `Serialize`

## [0.1.0] - 2025-12-18

First release
