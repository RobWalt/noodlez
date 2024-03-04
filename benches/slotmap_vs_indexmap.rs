use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indexmap::IndexMap;
use slotmap::SlotMap;

slotmap::new_key_type! { struct IndexType; }

struct Word(String);

fn create_input<const N: usize>() -> [Word; N] {
    const ARRAY_REPEAT_VALUE: Word = Word(String::new());
    let mut book = [ARRAY_REPEAT_VALUE; N];

    for i in 0..N {
        book[i] = Word(i.to_string());
    }

    book
}

fn create_slotmap<const N: usize>(book: [Word; N]) -> SlotMap<IndexType, Word> {
    let mut sm = SlotMap::with_key();
    for word in book {
        sm.insert(word);
    }
    sm
}

fn create_indexmap<const N: usize>(book: [Word; N]) -> IndexMap<usize, Word> {
    let mut im = IndexMap::new();
    for (i, word) in book.into_iter().enumerate() {
        im.insert(i, word);
    }
    im
}

// This is probably too naive although it does show what was expected. We might have to factor out
// everything that's not supposed to be measured a bit more and I have to admit I'm not really an
// expert for benchmarks
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(format!("create slotmap 100").as_str(), |b| {
        b.iter(|| create_slotmap(black_box(create_input::<100>())))
    });
    c.bench_function(format!("create indexmap 100").as_str(), |b| {
        b.iter(|| create_indexmap(black_box(create_input::<100>())))
    });

    c.bench_function(format!("create slotmap 100000").as_str(), |b| {
        b.iter(|| create_slotmap(black_box(create_input::<100000>())))
    });
    c.bench_function(format!("create indexmap 100000").as_str(), |b| {
        b.iter(|| create_indexmap(black_box(create_input::<100000>())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
