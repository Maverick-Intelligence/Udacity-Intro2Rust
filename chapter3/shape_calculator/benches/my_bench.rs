use criterion::{Criterion, criterion_group, criterion_main};
use shape_calculator::shapes::{circle::Circle, rectangle::Rectangle, square::Square};
use std::hint::black_box;

fn benchmark_circle_area(c: &mut Criterion) {
    c.bench_function("circle_area", |b| b.iter(|| Circle::new(black_box(5.0))));
}

fn benchmark_rectangle_area(c: &mut Criterion) {
    c.bench_function("rectangle_area", |b| {
        b.iter(|| Rectangle::new(black_box(3.0), black_box(4.0)))
    });
}

fn benchmark_square_area(c: &mut Criterion) {
    c.bench_function("square_area", |b| b.iter(|| Square::new(black_box(5.0))));
}

criterion_group!(
    benches,
    benchmark_circle_area,
    benchmark_rectangle_area,
    benchmark_square_area
);
criterion_main!(benches);
