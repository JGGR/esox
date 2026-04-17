## [0.1.3] - Unreleased

### Added

- Add `csv::load` module
  - New `load_csv_X(...)` methods to wrap over the errors of deser/parse steps
  - New `load_X(...)` methods using `InputFormat` instead of being generic over `RecordCsv` traits
- Add `pub fn parse_recordcsv<T>(...)`, `pub fn check_recordcsv<T>(...)` for full calc input types
  - `domain::niseci::{RiferimentoNISECI, CampionamentoNISECI, AnagraficaNISECI}`
  - `domain::hfbi::{CampionamentoHFBI, AnagraficaHFBI}`
- Add `csv::parser::niseci::{RiferimentoNISECIParseResult, CampionamentoNISECIParseResult}`
  - Wraps over old `parse_recordcsv_{riferimento, campionamento}_niseci()` return value, which used tuples
- Add `csv::parser::hfbi::CampionamentoHFBIParseResult`
  - Wraps over old `parse_recordcsv_campionamento_hfbi()` return value, which used tuples
- Add `get_x3_a()`, `get_x3_b()` to `RisultatoNISECI`
- Add `to_csv()` for:
  - `domain::niseci::{RisultatoNISECI, ValoriIntermediNISECI}`
  - `domain::hfbi::{RisultatoHFBI, ValoriIntermediHFBI}`
- Add `impl From<(f32, &AreaNISECI)>` to `domain::niseci::StatoEcologicoNISECI`
- Add `impl From<f32>` to `domain::hfbi::StatoEcologicoHFBI`
- Add `impl Default` to `domain::niseci::{ClassiEtaSpecieNISECI, ClassiEtaAlieniNISECI, InfoPopolazioniNISECI}`
- Add new methods to `CampionamentoHFBI`
  - `sort_by_peso_desc(&mut self)`
  - `sorted_by_peso_desc(&self) -> impl Iterator<Item = &RecordHFBI>`
- Add `meta::version()` to get crate version

### Changed

- Fix: ensure proper sorting inside `engines::hfbi::ddom::calc_ddom()`
- Fix: ensure proper sorting inside `domain::hfbi::CampionamentoHFBI::new()`
  - To avoid invalid instances (not sorted by descending `peso` we have to fully control construction
  - `v0.2` will change visibility of fields
- Deprecation warnings for parsing of `RiferimentoNISECI`, next version will change public API:
  - Out: will return `RiferimentoNISECI` on success branches (was: `Vec<SpecieNISECI>` on success branches)
- Deprecation warnings for parsing of `CampionamentoNISECI`, next version will change public API:
  - In: will expect `riferimento_specie` as `&RiferimentoNISECI` (was: `Vec<SpecieNISECI>`)
  - Out: will return `CampionamentoNISECI` on success branches (was: `Vec<RecordNISECI>` on success branches)
- Deprecation warnings for parsing of `CampionamentoHFBI`, next version will change public API:
  - Out: will return `CampionamentoHFBI` on success branches (was: `Vec<RecordHFBI>` on success branches)
- Deprecation warnings for visibility of `RiferimentoNISECI`, `CampionamentoNISECI`, `CampionamentoHFBI` fields
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
