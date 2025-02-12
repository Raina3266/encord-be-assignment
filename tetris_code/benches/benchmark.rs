use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use tetris::{grid::Grid, shape::Shape};

fn add_shapes(c: &mut Criterion) {
    c.bench_function("insert_and_clear", |bencher| {
        bencher.iter(|| {
            let mut grid = Grid::new(10);

            for _ in 0..1000 {
                grid.add_shape(Shape::Q, 0);
                grid.add_shape(Shape::Q, 2);
                grid.add_shape(Shape::Q, 4);
                grid.add_shape(Shape::Q, 6);
            }

            for _ in 0..1000 {
                grid.add_shape(Shape::Q, 8);
            }

            // prevent the compiler optimizing too much
            black_box(grid);
        });
    });
}

criterion_group!(benches, add_shapes);
criterion_main!(benches);
