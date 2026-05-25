use criterion::{Criterion, criterion_group, criterion_main};
use string_search_benchmark::{search_contains, search_find};

fn produce_haystack_needle() -> (String, &'static str) {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
                 sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                 Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
                 nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
                 reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
                 pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
                 culpa qui officia deserunt mollit anim id est laborum. ";

    let needle = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
                  sed do eiusmod tempor incididunt ut labore et dolore magna aliquaXYZZY42";

    let haystack = format!("{}{}", lorem.repeat(1000000), needle);
    (haystack, needle)
}

fn benchmark_search_contains(c: &mut Criterion) {
    let (haystack, needle) = produce_haystack_needle();

    c.bench_function("search_contains", |b| {
        b.iter(|| {
            search_contains(
                std::hint::black_box(&haystack),
                std::hint::black_box(needle),
                std::hint::black_box(false),
            )
        })
    });
}

fn benchmark_search_find(c: &mut Criterion) {
    let (haystack, needle) = produce_haystack_needle();

    c.bench_function("search_find", |b| {
        b.iter(|| {
            search_find(
                std::hint::black_box(&haystack),
                std::hint::black_box(needle),
                std::hint::black_box(false),
            )
        })
    });
}

criterion_group!(benches, benchmark_search_contains, benchmark_search_find);
criterion_main!(benches);
