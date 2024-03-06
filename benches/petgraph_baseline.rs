use criterion::{black_box, criterion_group, criterion_main, Criterion};
use petgraph::Graph;

fn create_graph<const N: usize>() -> Graph<(), ()> {
    todo!()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create a basic graph", |b| b.iter(|| todo!()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
