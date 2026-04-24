// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2026 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use esox::csv::deser::hfbi::check_campionamento_hfbi_reader;
use esox::csv::deser::hfbi::PlainRecordCsvCampionamentoHFBI;
use esox::csv::CAMPIONAMENTO_HFBI_HEADER;
use esox::domain::hfbi::{AnagraficaHFBI, HabitatHFBI, StagioneHFBI, TipoLagunaCostieraHFBI};
use esox::domain::location::Location;
use esox::domain::posf32::PositiveF32;
use esox::engines::hfbi::full::calculate_hfbi;
use esox::parser::hfbi::check_records_campionamento_hfbi;
use std::hint::black_box;
use std::io::Cursor;
use std::time::Duration;

fn make_input_campionamento(n: usize) -> String {
    (0..n)
        .map(|_| "GHG,25,240".to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_raw_input(n: usize) -> Vec<u8> {
    let camp_data = make_input_campionamento(n);
    let camp = format!(
        "{}\n{}",
        CAMPIONAMENTO_HFBI_HEADER
            .replace(",", ".")
            .replace(";", ","),
        camp_data
    );
    camp.into_bytes()
}

fn bench_name(tag: &str) -> String {
    const BACKEND: &'static str = "CSV v2-dev";
    format!("{}: {}", BACKEND, tag,)
}

fn full_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group(&bench_name("hfbi"));
    let has_headers = true;

    for &n in &[10, 50, 100, 500, 1_000, 5_000, 10_000] {
        let raw_camp = make_raw_input(n);

        group.throughput(Throughput::Elements(n as u64));

        // -------------------------
        // DESER PHASE
        // -------------------------
        group.bench_with_input(
            BenchmarkId::new("deser (campionamento)", n),
            &raw_camp,
            |b, c| {
                b.iter(|| {
                let camp = check_campionamento_hfbi_reader::<_, PlainRecordCsvCampionamentoHFBI>(
                    Cursor::new(black_box(c)),
                    has_headers,
                );

                black_box(camp)
            })
            },
        );

        // -------------------------
        // PARSE PHASE
        // -------------------------
        group.bench_with_input(
            BenchmarkId::new("deser + parse (campionamento)", n),
            &raw_camp,
            |b, c| {
                b.iter(|| {
                    let camp_d = check_campionamento_hfbi_reader::<
                        _,
                        PlainRecordCsvCampionamentoHFBI,
                    >(Cursor::new(black_box(c)), has_headers)
                    .expect("Input should be valid");
                    let camp = check_records_campionamento_hfbi(black_box(camp_d));
                    black_box(camp)
                })
            },
        );

        let camp_d_2 = check_campionamento_hfbi_reader::<_, PlainRecordCsvCampionamentoHFBI>(
            Cursor::new(&raw_camp),
            has_headers,
        )
        .expect("Input should be valid");

        let camp = check_records_campionamento_hfbi(camp_d_2).expect("Input should be valid");

        // -------------------------
        // COMPUTE PHASE
        // -------------------------
        let posf32 = PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32");
        let anagrafica = AnagraficaHFBI::new(
            "A".to_string(),
            "A".to_string(),
            Location {
                regione: "A".to_string(),
                provincia: "A".to_string(),
            },
            "A".to_string(),
            TipoLagunaCostieraHFBI::MAt1,
            StagioneHFBI::Primavera,
            HabitatHFBI::Vegetato,
            posf32,
            posf32,
        );
        group.bench_with_input(BenchmarkId::new("compute", n), &camp, |b, c| {
            b.iter(|| {
                let res = calculate_hfbi(black_box(c), black_box(&anagrafica));
                black_box(res)
            })
        });
    }

    group.finish();
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(70))
}

criterion_group! { name = benches;
    config = custom_criterion();
    targets = full_bench
}
criterion_main!(benches);
