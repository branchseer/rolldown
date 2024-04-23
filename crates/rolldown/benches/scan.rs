use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rolldown::{scan, ScanOutput};
use rolldown_common::ModuleType;

fn criterion_benchmark(c: &mut Criterion) {
  let source = Arc::<str>::from(include_str!("three_v0.163.0.mjs"));
  let path = Arc::<str>::from("three_v0.163.0.mjs");

  let scan_output = scan(path.clone(), ModuleType::EsmMjs, 0.into(), &source);
  let scan_output_bytes = bincode::encode_to_vec(scan_output, bincode::config::standard()).unwrap();

  c.bench_function("scan", |b| {
    b.iter(|| scan(path.clone(), ModuleType::EsmMjs, 0.into(), black_box(&source)))
  });
  c.bench_function("deserialize_scan_output", |b| {
    b.iter(|| {
      bincode::decode_from_slice::<ScanOutput, _>(
        black_box(&scan_output_bytes),
        bincode::config::standard(),
      )
      .unwrap()
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
