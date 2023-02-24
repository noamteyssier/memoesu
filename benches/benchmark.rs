use criterion::{criterion_group, criterion_main, Criterion};
use hashbrown::HashSet;
use petgraph::graph::NodeIndex;

/// takes the union of two sets
fn append_exclusive_orig(cnh: &HashSet<NodeIndex>, exc: &HashSet<NodeIndex>) -> HashSet<NodeIndex> {
    cnh.union(exc).copied().collect::<HashSet<NodeIndex>>()
}

/// takes the union of two sets by cloning the first set and extending it with the second
fn append_exclusive_new(cnh: &HashSet<NodeIndex>, exc: &HashSet<NodeIndex>) -> HashSet<NodeIndex> {
    let mut new = cnh.clone();
    new.extend(exc);
    new
}

fn overwrite_extension(
    exc: &HashSet<NodeIndex>,
    ext: &HashSet<NodeIndex>,
    v: &NodeIndex,
    w: &NodeIndex,
) -> HashSet<NodeIndex> {
    ext.union(exc)
        .filter(|u| *u > v)
        .filter(|u| *u != w)
        .cloned()
        .collect::<HashSet<NodeIndex>>()
}

fn overwrite_extension_new(
    exc: &HashSet<NodeIndex>,
    ext: &HashSet<NodeIndex>,
    v: &NodeIndex,
    w: &NodeIndex,
) -> HashSet<NodeIndex> {
    let mut new = ext.clone();
    new.remove(w);
    new.extend(exc.iter().filter(|u| *u > v));
    new
}

pub fn benchmark_append_exclusive(c: &mut Criterion) {
    let cnh = (0..8)
        .map(|i| NodeIndex::new(i))
        .collect::<HashSet<NodeIndex>>();
    let exc = (7..10)
        .map(|i| NodeIndex::new(i))
        .collect::<HashSet<NodeIndex>>();
    c.bench_function("append_exclusive_orig", |b| {
        b.iter(|| {
            append_exclusive_orig(&cnh, &exc);
        })
    });
    c.bench_function("append_exclusive_new", |b| {
        b.iter(|| {
            append_exclusive_new(&cnh, &exc);
        })
    });

    assert_eq!(
        append_exclusive_orig(&cnh, &exc),
        append_exclusive_new(&cnh, &exc)
    );
}

pub fn benchmark_overwrite_extension(c: &mut Criterion) {
    let exc = (7..10)
        .map(|i| NodeIndex::new(i))
        .collect::<HashSet<NodeIndex>>();
    let ext = (1..8)
        .map(|i| NodeIndex::new(i))
        .collect::<HashSet<NodeIndex>>();
    let v = NodeIndex::new(0);
    let w = NodeIndex::new(6);

    let mut orig = overwrite_extension(&exc, &ext, &v, &w)
        .iter()
        .map(|i| i.index())
        .collect::<Vec<usize>>();
    let mut new = overwrite_extension_new(&exc, &ext, &v, &w)
        .iter()
        .map(|i| i.index())
        .collect::<Vec<usize>>();
    orig.sort();
    new.sort();
    assert_eq!(orig, new);

    // assert_eq!(overwrite_extension(&exc, &ext, &v, &w), overwrite_extension_new(&exc, &ext, &v, &w));

    c.bench_function("overwrite_extension_orig", |b| {
        b.iter(|| {
            overwrite_extension(&exc, &ext, &v, &w);
        })
    });
    c.bench_function("overwrite_extension_new", |b| {
        b.iter(|| {
            overwrite_extension_new(&exc, &ext, &v, &w);
        })
    });
}

// criterion_group!(benches, benchmark_append_exclusive);
criterion_group!(benches, benchmark_overwrite_extension);
criterion_main!(benches);
