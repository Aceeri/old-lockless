
extern crate voxel;
#[macro_use]
extern crate criterion;

use voxel::chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, CHUNK_LENGTH, LocalBlockPosition};

use criterion::Criterion;
use criterion::black_box;

fn surrounding_block(x: usize, y: usize, z: usize) {
    let position = LocalBlockPosition::unchecked_new(x, y, z);
    let surrounding = position.surrounding();
}

fn surrounding_chunk() {
    for x in 0..(CHUNK_WIDTH - 1) {
        for y in 0..(CHUNK_HEIGHT - 1) {
            for z in 0..(CHUNK_LENGTH - 1) {
                let position = LocalBlockPosition::unchecked_new(x, y, z);
                let surrounding = position.surrounding();
                black_box(surrounding);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut positions = (0..CHUNK_WIDTH - 1)
        .flat_map(move |x| 
            (0..CHUNK_HEIGHT - 1)
                .flat_map(move |y|
                    (0..CHUNK_LENGTH - 1) .map(move |z| (x, y, z))
                )
        )
        .cycle();

    c.bench_function("surrounding_block", move |b| b.iter(|| {
        let position = positions.next().unwrap();
        surrounding_block(position.0, position.1, position.2);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
