## [0.1.6] - Unreleased

### Added

- Add `csv::deser::{RecordCsv, CommaDelimiter, SemicolonDelimiter}`
  - Will allow compile-type definition of csv delimiter for impls of:
    - `deser::Record{Riferimento,Campionamento,Anagrafica}NISECI`
    - `deser::Record{Campionamento,Anagrafica}HFBI`

### Changed

- Corrected deprecation message for `csv::parser::niseci::{parse,check}_anagrafica_niseci`
- Deprecation warning for visibility of `csv::parser::hfbi::{parse,check}_anagrafica_hfbi`

## [0.1.5] - 2026-04-30

### Added

- Add `csv::deser::utils::diagnostic` module
  - Provides generic formatting of errors
  - `utils::diagnostic::ww` module
    - Provides english formatting of errors

- Add `csv::stanis::giorgio` module
    - Provides italian formatting of errors

### Changed

- Implicit error logging now uses `stanis::giorgio::format_csv_errors`
  - May result in minor differences in output
  - Next version will drop the implicit logging
    - Usercode can use the `giorgio` format function explicitly instead
- Moved tests checking italian diagnostic messages to `stanis::giorgio::tests`
- Deprecation warning for visibility of `csv::deser::translate_error_message`
- Deprecation warning for visibility of `csv::deser::process_csv_errors`
- Deprecation warning for visibility of `deser::validate_serialized_records`
- Deprecation warnings for `csv::parser` module

## [0.1.4] - 2026-04-26

### Added

- Add `csv::deser::CsvConfig` to support explicit delimiter settings
- Add `csv::stanis` module

### Changed

- Deprecation warnings for visibility of `csv::deser::{niseci,hfbi}::VeryItalianRecordCsv*` structs
- Deprecation warnings for check functions in `csv::{deser,load}::{niseci,hfbi}`, next version will change public API:
  - In: will expect `delimiter` as new required argument
- Catch negative or not finite area values in:
  - `engines::niseci::x2::{calculate_sommatoria_x2_b, calculate_sommatoria_x2_b_per_alloctone}`
  - `engines::hfbi::bbent::calc_bbent`
  - `engines::hfbi::dbent::calc_dbent`
  - `engines::hfbi::ddom::calc_ddom`
  - `engines::hfbi::dhzp::calc_dhzp`
  - `engines::hfbi::dmig::calc_dmig`
  - Listed methods now return `Result`

## [0.1.3] - 2026-04-24

### Added

- Add `csv::load` module
  - New `load_csv_X(...)` methods to wrap over the errors of deser/parse steps
  - New `load_X(...)` methods using `InputFormat` instead of being generic over `RecordCsv` traits
- Add `pub fn parse_records<T>(...)`, `pub fn check_records<T>(...)` for full calc input types
  - `domain::niseci::{RiferimentoNISECI, CampionamentoNISECI, AnagraficaNISECI}`
  - `domain::hfbi::{CampionamentoHFBI, AnagraficaHFBI}`
- Add `deser` module
  - New `{parse,validate}_serialized_records()`, more generic
  - Holds refactored `Record` traits and `TipoRecord` enum
  - Previous definitions in `csv` are now a deprecated reexport
- Add `parser` module
  - Holds refactored `RecordError` enums
  - Holds renamed `parse_records` methods
  - Holds concrete `check_records` methods, previous definitions in `csv::parser` are now thin wrappers
- Add `parser::niseci::{RiferimentoNISECIParseResult, CampionamentoNISECIParseResult}`
  - Wraps over old `parse_recordcsv_{riferimento, campionamento}_niseci()` return value, which used tuples
- Add `parser::hfbi::CampionamentoHFBIParseResult`
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
- Add checked construction to `AnagraficaNISECI`, `AnagraficaHFBI`
  - Uses thin `PositiveF32` (refuses <=0, NaN, infinities)
- Add `meta::version()` to get crate version
- Add tests for order invariant on `CampionamentoHFBI` access/iteration methods
- Add tests for order invariant on `calc_ddom()`, `calculate_hfbi()`, `calculate_mmi()`, `calc_s90_b90()`
- Add tests for weird area cases for `calculate_niseci()`, `calculate_hfbi()`, using template files with explicit setting of `Anagrafica{NISECI,HFBI}` width/length fields
  - Zero
  - Negative
  - Infinity / negative infinity
  - Quiet NaN (using likely quiet NaN from `f32::NAN`)
- Add empty `impl std::error::Error` for most custom errors
- Add `experimental` feature
  - For now it guards `Deserialize` on input types since it's not properly tested
- Add `json` feature
  - For now it provides `json::{deser,load}` modules

### Changed

- Fix: ensure proper sorting inside `engines::hfbi::ddom::calc_ddom()`
- Fix: ensure proper sorting inside `domain::hfbi::CampionamentoHFBI::new()`
  - To avoid invalid instances (not sorted by descending `peso`) we have to fully control construction
  - `v0.2` will change visibility of fields
- Deprecation warnings for parsing of `RiferimentoNISECI`, next version will change public API:
  - Out: will return `RiferimentoNISECI` on success branches (was: `Vec<SpecieNISECI>` on success branches)
- Deprecation warnings for parsing of `CampionamentoNISECI`, next version will change public API:
  - In: will expect `riferimento_specie` as `&RiferimentoNISECI` (was: `Vec<SpecieNISECI>`)
  - Out: will return `CampionamentoNISECI` on success branches (was: `Vec<RecordNISECI>` on success branches)
- Deprecation warnings for parsing of `CampionamentoHFBI`, next version will change public API:
  - Out: will return `CampionamentoHFBI` on success branches (was: `Vec<RecordHFBI>` on success branches)
- Deprecation warnings for visibility of `RiferimentoNISECI`, `CampionamentoNISECI`, `CampionamentoHFBI` fields
- Deprecation warnings for visibility of `AnagraficaNISECI`, `AnagraficaHFBI` length/width fields
- Deprecation warnings for visibility of `csv::RecordCsv*` traits, `csv::TipoRecordCsv`
- Deprecation warnings for visibility of `csv::parser::{niseci,hfbi}::RecordCsv*Error` error enums
- Drop `once_cell` dependency by replacing `once_cell::sync::Lazy` with `std::sync::LazyLock`
  - Needs `rust 1.80`
- Make types related to full calc results `Deserialize`
- Make types related to full calc input `Serialize`, `Deserialize` (experimental)
  - Custom deser for records field of `CampionamentoHFBI` (experimental)
  - Custom deser for length/width fields of `AnagraficaNISECI` and `AnagraficaHFBI` (experimental)
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
