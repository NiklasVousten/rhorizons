#![allow(unused)]

use chrono::{TimeZone, Utc};
use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rhorizons::{parse, EphemerisOrbitalElementsItem, EphemerisOrbitalElementsParser, query};

fn old_parsing(data: &String) {
    let ephem: Vec<_> = EphemerisOrbitalElementsParser::parse(data.lines()).collect();
}

fn bench_parsing(c: &mut Criterion) {
    let id = 399;
    let start_time = Utc.with_ymd_and_hms(2016, 10, 15, 12, 0, 0).unwrap();
    let stop_time = Utc.with_ymd_and_hms(2018, 10, 15, 12, 0, 0).unwrap();

    let local = include_str!("../src/orbital_elements.txt").to_string();

    let online = tokio::runtime::Runtime::new().unwrap().block_on(query(&[
        ("COMMAND", id.to_string().as_str()),
        // Select Sun as a observer. Note that Solar System Barycenter is in a
        // slightly different place.
        // https://astronomy.stackexchange.com/questions/44851/
        ("CENTER", "500@10"),
        ("EPHEM_TYPE", "ELEMENTS"),
        // https://ssd.jpl.nasa.gov/horizons/manual.html#time
        (
            "START_TIME",
            start_time.format("%Y-%b-%d-%T").to_string().as_str(),
        ),
        (
            "STOP_TIME",
            stop_time.format("%Y-%b-%d-%T").to_string().as_str(),
        ),
    ])).unwrap().join("\n");

    let mut group = c.benchmark_group("Parsing");

    group.bench_with_input(BenchmarkId::new("Custom", "Local"), &local, |b, d| {
        b.iter(|| old_parsing(d))
    });
    group.bench_with_input(BenchmarkId::new("Custom", "Online"), &online, |b, d| {
        b.iter(|| old_parsing(d))
    });
    group.bench_with_input(BenchmarkId::new("Chumsky", "Local"), &local,
        |b, d| b.iter(|| parse(d.as_str())));
    group.bench_with_input(BenchmarkId::new("Chumsky", "Online"), &online,
        |b, d| b.iter(|| parse(d.as_str())));
    group.finish();
}

criterion_group!(benches, bench_parsing);
criterion_main!(benches);
