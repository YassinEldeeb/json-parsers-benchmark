use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustc_serialize::json::Json;
use serde_json::Value;
use simd_json::prelude::*;
use std::fs::File;
use std::io::Read;

pub fn read_json_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
    Ok(json_str)
}

pub fn parse_json(c: &mut Criterion) {
    let json_data = read_json_file("./benches/data.json").expect("Failed to read JSON file");

    let mut json_vec = Vec::from(&json_data[..]);

    c.bench_function("simd_json", |b| {
        b.iter(|| {
            let parsed = simd_json::to_borrowed_value(&mut json_vec).unwrap();
            let users = parsed.get("data").and_then(|v| v.get("users")).unwrap();
            black_box(users);
        })
    });

    c.bench_function("serde_json", |b| {
        b.iter(|| {
            let parsed: Value = serde_json::from_slice(&json_vec).unwrap();
            let users = parsed.get("data").and_then(|v| v.get("users")).unwrap();
            black_box(users);
        })
    });

    c.bench_function("rustc-serialize", |b| {
        b.iter(|| {
            let parsed = Json::from_str(&json_data).expect("Failed to parse JSON");
            let users = parsed.find_path(&["data", "users"]).unwrap();
            black_box(users);
        })
    });
}

criterion_group!(benches, parse_json);
criterion_main!(benches);
