
use crate::block::{Block, BlockSize};

pub const CHUNK_HEIGHT: usize = 64; // Y
pub const CHUNK_WIDTH: usize = 64; // X
pub const CHUNK_LENGTH: usize = 64; // Z
pub const CHUNK_SIZE: usize = CHUNK_HEIGHT * CHUNK_WIDTH * CHUNK_LENGTH;

pub const Y_SIZE: usize = CHUNK_HEIGHT * CHUNK_WIDTH;
pub const X_SIZE: usize = CHUNK_WIDTH;
pub const Z_SIZE: usize = 1;

pub const EMPTY_BLOCK: Block = Block::hard_create(0);

pub const fn chunk_index(x: usize, y: usize, z: usize) -> usize {
    (y * Y_SIZE) + (x * X_SIZE) + (z * Z_SIZE)
}

#[derive(Debug, Copy, Clone)]
pub struct LocalBlockPosition {
    x: usize,
    y: usize,
    z: usize,
}

impl LocalBlockPosition {
    pub fn new(x: usize, y: usize, z: usize) -> Option<LocalBlockPosition> {
        if x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_LENGTH {
            return None;
        }

        Some(LocalBlockPosition { x, y, z })
    }

    pub fn unchecked_new(x: usize, y: usize, z: usize) -> LocalBlockPosition {
        LocalBlockPosition { x, y, z }
    }

    pub fn index(&self) -> usize {
        chunk_index(self.x as usize, self.y as usize, self.z as usize)
    }

    pub fn possible_surrounding(&self) -> [LocalBlockPosition; 6] {
        [
            LocalBlockPosition { x: self.x + 1, y: self.y, z: self.z },
            LocalBlockPosition { x: self.x, y: self.y + 1, z: self.z },
            LocalBlockPosition { x: self.x, y: self.y, z: self.z + 1 },
            LocalBlockPosition { x: self.x - 1, y: self.y, z: self.z },
            LocalBlockPosition { x: self.x, y: self.y - 1, z: self.z },
            LocalBlockPosition { x: self.x, y: self.y, z: self.z - 1 },
        ]
    }

    pub fn surrounding(&self) -> (usize, [LocalBlockPosition; 6]) {
        let mut positions = self.possible_surrounding();
        let mut valid = 0b0011_1111;
        if self.x + 1 >= CHUNK_WIDTH {
            valid |= 0b0000_0001;
        }

        if self.y + 1 >= CHUNK_HEIGHT {
            valid |= 0b0000_0010;
        }

        if self.z + 1 >= CHUNK_LENGTH {
            valid |= 0b0000_0100;
        }

        if self.x == 0 {
            valid |= 0b0000_1000;
        }

        if self.y == 0 {
            valid |= 0b0001_0000;
        }

        if self.z == 0 {
            valid |= 0b0010_0000;
        }

        (valid, positions)
    }

    //pub fn from_index(index: usize) -> Option<LocalBlockPosition> {
        //if index > 0 && index < CHUNK_SIZE {
            //return Some(LocalBlockPosition(index))
        //}

        //None
    //}
}

pub trait Chunk {
    fn block(&self, position: &LocalBlockPosition) -> Block;
}

pub trait ChunkMut {
    fn set_block(&mut self, position: &LocalBlockPosition, block: Block);
}

pub struct BoxedChunk {
    blocks: Box<[Block]>,
}

impl BoxedChunk {
    pub fn empty() -> BoxedChunk {
        BoxedChunk {
            blocks: vec![EMPTY_BLOCK; CHUNK_SIZE as usize].into_boxed_slice(),
        }
    }

    pub fn get_ref(&self) -> ChunkRef<'_> {
        ChunkRef {
            blocks: &self.blocks,
        }
    }
}

impl Chunk for BoxedChunk {
    fn block(&self, position: &LocalBlockPosition) -> Block {
        self.blocks[position.index() as usize]
    }
}

impl ChunkMut for BoxedChunk {
    fn set_block(&mut self, position: &LocalBlockPosition, block: Block) {
        self.blocks[position.index() as usize] = block;
    }
}

pub struct ChunkRef<'a> {
    // Stored in yxz order for caching (we will probably hit horizontal axis together).
    blocks: &'a [Block],
}

impl<'a> Chunk for ChunkRef<'a> {
    fn block(&self, position: &LocalBlockPosition) -> Block {
        self.blocks[position.index() as usize]
    }
}

impl<'a> ChunkRef<'a> {
    //pub fn positions(&self) -> Vec<LocalBlockPosition> {
        //let mut positions = Vec::new();
        //for y in 0..CHUNK_HEIGHT {
            //for x in 0..CHUNK_WIDTH {
                //for z in 0..CHUNK_LENGTH {
                    //positions.push(LocalBlockPosition { y, x, z });
                //}
            //}
        //}

        //positions
    //}

    //pub fn positions_iter(&self) -> impl Iterator<Item = LocalBlockPosition> {
        //(0..CHUNK_HEIGHT)
            //.flat_map(move |height| {
                //(0..CHUNK_WIDTH).flat_map(move |width| {
                    //(0..CHUNK_LENGTH).map(move |length| {
                        //(height, width, length)
                    //})
                //})
            //})
            //.map(|(y, x, z)| LocalBlockPosition { y, x, z })
    //}

    //pub fn mesh(&self) -> ChunkMesh {
    //}
}

#[cfg(test)]
mod test {
    use crate::chunk::{Chunk, ChunkMut, BoxedChunk, ChunkRef, LocalBlockPosition, CHUNK_HEIGHT, CHUNK_WIDTH, CHUNK_LENGTH, CHUNK_SIZE, Y_SIZE, X_SIZE, Z_SIZE};
    use crate::block::{Block, BlockSize, MAX_BLOCK_ID};
    use std::collections::HashSet;

    // Sanity checking that setting and getting blocks refer to the same position.
    #[test]
    fn block_position() {
        let mut chunk = BoxedChunk::empty();

        let mut current = 0;
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_LENGTH {
                    let position = LocalBlockPosition::new(x, y, z).unwrap();

                    let created_block = Block::hard_create(current);
                    chunk.set_block(&position, created_block);
                    let block = chunk.block(&position);

                    assert_eq!(created_block, block);
                    current = ((current as usize % MAX_BLOCK_ID) + 1) as BlockSize;
                }
            }
        }
    }

    macro_rules! should_panic {
        ($($name:ident => $block:block),*) => {
            $(
                #[test]
                #[should_panic]
                fn $name() {
                    let () = $block;
                }
            )*
        }
    }

    should_panic! {
        x_out_of_bounds => { LocalBlockPosition::new(CHUNK_WIDTH as usize, 0, 0).unwrap(); },
        y_out_of_bounds => { LocalBlockPosition::new(0, CHUNK_HEIGHT as usize, 0).unwrap(); },
        z_out_of_bounds => { LocalBlockPosition::new(0, 0, CHUNK_LENGTH as usize).unwrap(); }
    }

    #[test]
    fn valid_positions() {
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_LENGTH {
                    LocalBlockPosition::new(x, y, z).unwrap();
                }
            }
        }
    }

    #[test]
    fn surrounding_positions() {
        struct SurroundingCoverage {
            pub set: HashSet<(usize, usize, usize)>,
        }

        impl SurroundingCoverage {
            fn assert_surrounding(&mut self, position: (usize, usize, usize), count: usize) {
                let local_position = LocalBlockPosition::new(position.0, position.1, position.2).unwrap();
                assert_eq!(surrounding_count(local_position), count);
                self.set.insert(position);
            }
        }

        fn surrounding_count(position: LocalBlockPosition) -> usize {
            let (valid, positions) = position.surrounding();
            positions.len()
            //position.surrounding().iter().filter_map(|p| p.as_ref()).count()
        }

        let mut coverage = SurroundingCoverage { set: HashSet::new(), };

        // Corners should have 3.
        coverage.assert_surrounding((CHUNK_WIDTH - 1,                0,                0), 3); // left bottom back
        coverage.assert_surrounding((CHUNK_WIDTH - 1,                0, CHUNK_LENGTH - 1), 3); // left bottom front
        coverage.assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1,                0), 3); // left top back
        coverage.assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1, CHUNK_LENGTH - 1), 3); // left top front
        coverage.assert_surrounding((              0,                0,                0), 3); // right bottom back
        coverage.assert_surrounding((              0,                0, CHUNK_LENGTH - 1), 3); // right bottom front
        coverage.assert_surrounding((              0, CHUNK_HEIGHT - 1,                0), 3); // right top back
        coverage.assert_surrounding((              0, CHUNK_HEIGHT - 1, CHUNK_LENGTH - 1), 3); // right top front

        // Edges should have 4
        for x in 1..(CHUNK_WIDTH - 1) {
            coverage.assert_surrounding((              x,                0,               0), 4);
        }

        for z in 1..(CHUNK_LENGTH - 1) {
            coverage.assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1,               z), 4);
            coverage.assert_surrounding((CHUNK_WIDTH - 1,                0,               z), 4);
            coverage.assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1,               z), 4);
            coverage.assert_surrounding((CHUNK_WIDTH - 1,                0,               z), 4);
            coverage.assert_surrounding((              0, CHUNK_HEIGHT - 1,               z), 4);
            coverage.assert_surrounding((              0,                0,               z), 4);
            coverage.assert_surrounding((              0, CHUNK_HEIGHT - 1,               z), 4);
            coverage.assert_surrounding((              0,                0,               z), 4);
        }

        for y in 1..(CHUNK_HEIGHT - 1) {
            coverage.assert_surrounding((CHUNK_WIDTH - 1,               y, CHUNK_LENGTH - 1), 4);
            coverage.assert_surrounding((CHUNK_WIDTH - 1,               y,                0), 4);
            coverage.assert_surrounding((              0,               y, CHUNK_LENGTH - 1), 4);
            coverage.assert_surrounding((              0,               y,                0), 4);
        }

        // Inner sides should have 5
        for y in 1..(CHUNK_HEIGHT - 1) {
            for z in 1..(CHUNK_LENGTH - 1) {
                coverage.assert_surrounding((CHUNK_WIDTH - 1, y, z), 5);
                coverage.assert_surrounding((              0, y, z), 5);
            }
        }

        for x in 1..(CHUNK_WIDTH - 1) {
            for z in 1..(CHUNK_LENGTH - 1) {
                coverage.assert_surrounding((x, CHUNK_HEIGHT - 1, z), 5);
                coverage.assert_surrounding((x,                0, z), 5);
            }
        }

        for x in 1..(CHUNK_WIDTH - 1) {
            for y in 1..(CHUNK_HEIGHT - 1) {
                coverage.assert_surrounding((x, y, CHUNK_LENGTH - 1), 5);
                coverage.assert_surrounding((x, y,                0), 5);
            }
        }

        // Interiors should have 6
        for x in 1..(CHUNK_WIDTH - 1) {
            for y in 1..(CHUNK_HEIGHT - 1) {
                for z in 1..(CHUNK_LENGTH - 1) {
                    coverage.assert_surrounding((x, y, z), 6);
                }
            }
        }

        let mut count = 0;
        for x in 0..(CHUNK_WIDTH - 1) {
            for y in 0..(CHUNK_HEIGHT - 1) {
                for z in 0..(CHUNK_LENGTH - 1) {
                    assert!(coverage.set.contains(&(x, y, z)));
                }
            }
        }
    }
}
