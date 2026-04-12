## [0.1.3] - Unreleased

### Added

- Add `csv::load` module
  - New methods wrap over the errors of deser/parse steps
- Add `get_x3_a()`, `get_x3_b()` to `RisultatoNISECI`

### Changed

- Moved templates data for tests `from tests::engines::{hfbi, niseci}::full` to `tests::test_utils`

## [0.1.2] - 2026-02-06

### Changed

- Refactor `RecordCsvCampionamentoNISECI` trait to have `peso` be `f32`
- Check and refuse `NaN` and `Inf` for `peso` for `RecordNISECI`, `RecordHFBI`
- Add `.github/dependabot.yml`
- Update copyright

## [0.1.1] - 2025-12-19

### Changed

- Make types related to full calc results Serialize

## [0.1.0] - 2025-12-18

First release
