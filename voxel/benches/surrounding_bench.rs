
extern crate voxel;
#[macro_use]
extern crate util;
#[macro_use]
extern crate criterion;

use std::io::Read;

use voxel::chunk::{Chunk, BoxedChunk, CHUNK_HEIGHT, CHUNK_WIDTH, CHUNK_LENGTH, LocalBlockPosition};
use voxel::block::{Block, BlockRegistry, BlockRegistryFile};

use criterion::Criterion;
use criterion::black_box;

fn surrounding_block(x: usize, y: usize, z: usize) -> (usize, [LocalBlockPosition; 6]) {
    let position = LocalBlockPosition::unchecked_new(x, y, z);
    position.surrounding()
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

fn surrounding(c: &mut Criterion) {
    let mut positions = (0..CHUNK_WIDTH - 1)
        .flat_map(move |x| 
            (0..CHUNK_HEIGHT - 1)
                .flat_map(move |y|
                    (0..CHUNK_LENGTH - 1).map(move |z| (x, y, z))
                )
        )
        .cycle();

    //c.bench_function("surrounding_block", move |b| b.iter(|| {
        //let position = positions.next().unwrap();
        //surrounding_block(position.0, position.1, position.2);
    //}));

    //c.bench_function("surrounding_chunk", move |b| b.iter(|| {
        //for x in 0..(CHUNK_WIDTH - 1) {
            //for y in 0..(CHUNK_HEIGHT - 1) {
                //for z in 0..(CHUNK_LENGTH - 1) {
                    //let (valid, surrounding) = black_box(surrounding_block(x, y, z));
                //}
            //}
        //}
    //}));
    
    let chunk = BoxedChunk::flat(Block::hard_create(16), 5);
    let (registry, failures) = BlockRegistry::from_file("resources/registry.json").unwrap();
    c.bench_function("visible_blocks", move |b| b.iter(|| {
        chunk.visible_blocks(&registry);
    }));
}

fn registry(c: &mut Criterion) {
    let mut file = std::fs::File::open("resources/registry.json").unwrap();
    let mut buffer = "".to_owned();
    file.read_to_string(&mut buffer).unwrap();

    let registry_file: BlockRegistryFile = serde_json::from_str(&buffer).unwrap();

    //c.bench_function("deserialize_registry", move |b| b.iter(|| {
        //BlockRegistry::from_str(black_box(&buffer));
    //}));

    //c.bench_function("convert_registryfile", move |b| b.iter(|| {
        //let mut registry = BlockRegistry::empty();
        //let failures = registry_file.into_registry(black_box(&mut registry));
    //}));
}

criterion_group!(benches, surrounding, registry);
criterion_main!(benches);
