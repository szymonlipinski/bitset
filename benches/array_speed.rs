use criterion::{criterion_group, criterion_main, Criterion};

#[inline]
fn simple_bits() -> u64 {
    let mut value: u64 = 0;

    // set all bits one by one
    for i in 0..63 {
        value = value | (1 << i);
    }
    // reset all bits one by one
    for i in 0..63 {
        value = value & !(1 << i);
    }
    value
}

#[inline]
fn array_bits() -> u64 {
    let mut value: [u64; 1] = [0];
    // set all bits one by one
    for i in 0..63 {
        value[0] = value[0] | (1 << i);
    }
    // reset all bits one by one
    for i in 0..63 {
        value[0] = value[0] & !(1 << i);
    }
    value[0]
}

#[inline]
fn vector_bits() -> u64 {
    let mut value: Vec<u64> = vec![0];
    // set all bits one by one
    for i in 0..63 {
        value[0] = value[0] | (1 << i);
    }
    // reset all bits one by one
    for i in 0..63 {
        value[0] = value[0] & !(1 << i);
    }
    value[0]
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("Bits");
    c.bench_function("simple_bits", |b| b.iter(|| simple_bits()));
    c.bench_function("array_bits", |b| b.iter(|| array_bits()));
    c.bench_function("vector_bits", |b| b.iter(|| vector_bits()));
    c.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
