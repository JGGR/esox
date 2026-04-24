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
use esox::csv::deser::niseci::{
    check_campionamento_niseci_reader, check_riferimento_niseci_reader,
};
use esox::csv::deser::niseci::{
    PlainRecordCsvCampionamentoNISECI, PlainRecordCsvRiferimentoNISECI,
};
use esox::csv::{CAMPIONAMENTO_NISECI_HEADER, RIFERIMENTO_NISECI_HEADER};
use esox::domain::location::Location;
use esox::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, ComunitaNISECI, IdroEcoRegioneNISECI, TipoComunitaNISECI,
};
use esox::domain::posf32::PositiveF32;
use esox::parser::niseci::{check_records_campionamento_niseci, check_records_riferimento_niseci};

use esox::engines::niseci::full::calculate_niseci;

use rand::rng;
use rand::seq::SliceRandom;
use std::hint::black_box;
use std::io::Cursor;
use std::time::Duration;

fn make_nice_input_riferimento(n: usize, prefix_len: usize) -> String {
    let prefix = "A".repeat(prefix_len);

    (0..n)
        .map(|i| {
            format!(
                "A,A,{}{:016},AUT,1,0,0,45,90,130,150,0.5,0.67,1.5,2,0.052246487,0.086500523",
                prefix, i
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_randorder_input_riferimento(n: usize, prefix_len: usize) -> String {
    let prefix = "A".repeat(prefix_len);

    let mut v = (0..n)
        .map(|i| {
            format!(
                "A,A,{}{:016},AUT,1,0,0,45,90,130,150,0.5,0.67,1.5,2,0.052246487,0.086500523",
                prefix, i
            )
        })
        .collect::<Vec<_>>();
    v.shuffle(&mut rng());
    v.join("\n")
}

fn make_nice_input_campionamento(n: usize, prefix_len: usize) -> String {
    let prefix = "A".repeat(prefix_len);

    (0..n)
        .map(|_| format!("1/1/1,A,1,{}{:016},275,152", prefix, n - 1))
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_randorder_success_input_campionamento(n: usize, prefix_len: usize) -> String {
    let prefix = "A".repeat(prefix_len);

    let mut v = (0..n)
        .map(|i| format!("1/1/1,A,1,{}{:016},275,152", prefix, i))
        .collect::<Vec<_>>();
    v.shuffle(&mut rng());
    v.join("\n")
}

fn make_input(
    n: usize,
    rif_builder: fn(usize, usize) -> String,
    camp_builder: fn(usize, usize) -> String,
) -> (Vec<u8>, Vec<u8>) {
    let prefix_len = PREFIX_LEN;
    let rif_data = rif_builder(n, prefix_len);
    let camp_data = camp_builder(n, prefix_len);
    let rif = format!(
        "{}\n{}",
        RIFERIMENTO_NISECI_HEADER
            .replace(",", ".")
            .replace(";", ","),
        rif_data
    );
    let camp = format!(
        "{}\n{}",
        CAMPIONAMENTO_NISECI_HEADER
            .replace(",", ".")
            .replace(";", ","),
        camp_data
    );
    (rif.into_bytes(), camp.into_bytes())
}

fn make_nice_input(n: usize) -> (Vec<u8>, Vec<u8>) {
    make_input(
        n,
        make_nice_input_riferimento,
        make_nice_input_campionamento,
    )
}

fn make_randorder_success_input(n: usize) -> (Vec<u8>, Vec<u8>) {
    make_input(
        n,
        make_randorder_input_riferimento,
        make_randorder_success_input_campionamento,
    )
}

fn run_bench(c: &mut Criterion, name: &str, build_input: fn(usize) -> (Vec<u8>, Vec<u8>)) {
    let mut group = c.benchmark_group(name);
    let has_headers = true;

    for &n in &[10, 50, 100, 500, 1_000, 5_000, 10_000] {
        let (raw_rif, raw_camp) = build_input(n);

        group.throughput(Throughput::Elements(n as u64));

        // -------------------------
        // DESER PHASE
        // -------------------------
        group.bench_with_input(
            BenchmarkId::new("deser (riferimento)", n),
            &raw_rif,
            |b, r| {
                b.iter(|| {
                    let rif = check_riferimento_niseci_reader::<_, PlainRecordCsvRiferimentoNISECI>(
                        Cursor::new(black_box(r)),
                        has_headers,
                    );
                    black_box(rif)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("deser (campionamento)", n),
            &raw_camp,
            |b, c| {
                b.iter(|| {
                    let camp = check_campionamento_niseci_reader::<
                        _,
                        PlainRecordCsvCampionamentoNISECI,
                    >(Cursor::new(black_box(c)), has_headers);

                    black_box(camp)
                })
            },
        );

        // -------------------------
        // PARSE PHASE
        // -------------------------
        group.bench_with_input(
            BenchmarkId::new("deser + parse (riferimento)", n),
            &raw_rif,
            |b, r| {
                b.iter(|| {
                    let rif_d =
                        check_riferimento_niseci_reader::<_, PlainRecordCsvRiferimentoNISECI>(
                            Cursor::new(black_box(r)),
                            has_headers,
                        )
                        .expect("Input should be valid");
                    let rif = check_records_riferimento_niseci(black_box(rif_d))
                        .expect("Input should be valid");
                    black_box(rif)
                })
            },
        );

        let rif_d = check_riferimento_niseci_reader::<_, PlainRecordCsvRiferimentoNISECI>(
            Cursor::new(&raw_rif),
            has_headers,
        )
        .expect("Input should be valid");
        let rif = check_records_riferimento_niseci(rif_d).expect("Input should be valid");

        group.bench_with_input(
            BenchmarkId::new("deser + parse (campionamento)", n),
            &raw_camp,
            |b, c| {
                b.iter(|| {
                    let camp_d = check_campionamento_niseci_reader::<
                        _,
                        PlainRecordCsvCampionamentoNISECI,
                    >(Cursor::new(black_box(c)), has_headers)
                    .expect("Input should be valid");
                    let camp =
                        check_records_campionamento_niseci(black_box(camp_d), black_box(&rif));

                    black_box(camp)
                })
            },
        );

        let rif_d_2 = check_riferimento_niseci_reader::<_, PlainRecordCsvRiferimentoNISECI>(
            Cursor::new(&raw_rif),
            has_headers,
        )
        .expect("Input should be valid");
        let camp_d_2 = check_campionamento_niseci_reader::<_, PlainRecordCsvCampionamentoNISECI>(
            Cursor::new(&raw_camp),
            has_headers,
        )
        .expect("Input should be valid");

        let elenco_specie =
            check_records_riferimento_niseci(rif_d_2).expect("Input should be valid");
        let rif = elenco_specie.clone();
        let camp = check_records_campionamento_niseci(camp_d_2, &elenco_specie)
            .expect("Input should be valid");

        // -------------------------
        // COMPUTE PHASE
        // -------------------------
        let posf32 = PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32");
        let anagrafica = AnagraficaNISECI::new(
            ComunitaNISECI::new(TipoComunitaNISECI::Redatta, None, None),
            "A".to_string(),
            "A".to_string(),
            AreaNISECI::Mediterranea,
            "A".to_string(),
            "A".to_string(),
            IdroEcoRegioneNISECI::Toscana,
            Location {
                regione: "A".to_string(),
                provincia: "A".to_string(),
            },
            posf32,
            posf32,
        );
        group.bench_with_input(BenchmarkId::new("compute", n), &(rif, camp), |b, (r, c)| {
            b.iter(|| {
                let res = calculate_niseci(black_box(c), black_box(r), black_box(&anagrafica));
                black_box(res)
            })
        });

        /*
        // -------------------------
        // ISOLATED JOIN (CRITICAL)
        // -------------------------
        group.bench_with_input(BenchmarkId::new("join_only n={}", n), &(rif_d.clone(), camp_d.clone()), |b, (r, c)| {
            b.iter(|| {
                join_by_id_naive(black_box(r), black_box(c))
            })
        });
        */
    }

    group.finish();
}

const PREFIX_LEN: usize = 16;

fn bench_name(tag: &str) -> String {
    const BACKEND: &'static str = "CSV v2-dev";
    format!("{}: {tag}, prefix: {PREFIX_LEN}", BACKEND)
}

fn full_bench(c: &mut Criterion) {
    run_bench(c, &bench_name("late join"), make_nice_input);
    run_bench(
        c,
        &bench_name("avg (shuffle)"),
        make_randorder_success_input,
    );
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(70))
}

criterion_group! { name = benches;
    config = custom_criterion();
    targets = full_bench
}
criterion_main!(benches);
