use criterion::{black_box, criterion_group, criterion_main, Criterion,};
use bra_ket::*;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    for n in (20..27).step_by(2) {
        let program = qft(n);
        let mut state = StateVector::new(n);
        c.bench_function(&*format!("qft{}", n), |b| b.iter(|| program.run(&mut state)));
    };
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);