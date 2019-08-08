
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

#[derive(Debug, Copy, Clone)]
pub struct P {
    x: isize,
    y: isize,
    z: isize,
}

impl LocalBlockPosition {
    pub fn new(x: usize, y: usize, z: usize) -> Option<LocalBlockPosition> {
        if x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_LENGTH {
            return None;
        }

        Some(LocalBlockPosition { x, y, z })
    }

    pub fn index(&self) -> usize {
        chunk_index(self.x, self.y, self.z)
    }

    pub fn surrounding(&self) -> [Option<LocalBlockPosition>; 6] {
        let mut positions = [
            LocalBlockPosition::new(self.x + 1, self.y, self.z),
            LocalBlockPosition::new(self.x, self.y + 1, self.z),
            LocalBlockPosition::new(self.x, self.y, self.z + 1),
            None,
            None,
            None,
        ];

        if self.x != 0 {
            positions[3] = LocalBlockPosition::new(self.x - 1, self.y, self.z);
        }
        if self.y != 0 {
            positions[4] = LocalBlockPosition::new(self.x, self.y - 1, self.z);
        }
        if self.z != 0 {
            positions[5] = LocalBlockPosition::new(self.x, self.y, self.z - 1);
        }

        //println!("{:?}", [
            //P { x: self.x as isize + 1 as isize, y: self.y as isize, z: self.z as isize },
            //P { x: self.x as isize, y: self.y as isize + 1 as isize, z: self.z as isize },
            //P { x: self.x as isize, y: self.y as isize, z: self.z as isize + 1 as isize },
            //P { x: self.x as isize - 1 as isize, y: self.y as isize, z: self.z as isize },
            //P { x: self.x as isize, y: self.y as isize - 1 as isize, z: self.z as isize },
            //P { x: self.x as isize, y: self.y as isize, z: self.z as isize - 1 as isize },
        //]);

        //println!("{:?}", positions);

        positions
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
            blocks: vec![EMPTY_BLOCK; CHUNK_SIZE].into_boxed_slice(),
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
        self.blocks[position.index()]
    }
}

impl ChunkMut for BoxedChunk {
    fn set_block(&mut self, position: &LocalBlockPosition, block: Block) {
        self.blocks[position.index()] = block;
    }
}

pub struct ChunkRef<'a> {
    // Stored in yxz order for caching (we will probably hit horizontal axis together).
    blocks: &'a [Block],
}

impl<'a> Chunk for ChunkRef<'a> {
    fn block(&self, position: &LocalBlockPosition) -> Block {
        self.blocks[position.index()]
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
        x_out_of_bounds => { LocalBlockPosition::new(CHUNK_WIDTH, 0, 0).unwrap(); },
        y_out_of_bounds => { LocalBlockPosition::new(0, CHUNK_HEIGHT, 0).unwrap(); },
        z_out_of_bounds => { LocalBlockPosition::new(0, 0, CHUNK_LENGTH).unwrap(); }
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
        fn assert_surrounding(position: (usize, usize, usize), count: usize) {
            let position = LocalBlockPosition::new(position.0, position.1, position.2).unwrap();
            assert_eq!(surrounding_count(position), count);
        }

        fn surrounding_count(position: LocalBlockPosition) -> usize {
            position.surrounding().iter().filter_map(|p| p.as_ref()).count()
        }

        // Corners should have 3.
        assert_surrounding((CHUNK_WIDTH - 1,                0,                0), 3); // left bottom back
        assert_surrounding((CHUNK_WIDTH - 1,                0, CHUNK_LENGTH - 1), 3); // left bottom front
        assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1,                0), 3); // left top back
        assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1, CHUNK_LENGTH - 1), 3); // left top front
        assert_surrounding((              0,                0,                0), 3); // right bottom back
        assert_surrounding((              0,                0, CHUNK_LENGTH - 1), 3); // right bottom front
        assert_surrounding((              0, CHUNK_HEIGHT - 1,                0), 3); // right top back
        assert_surrounding((              0, CHUNK_HEIGHT - 1, CHUNK_LENGTH - 1), 3); // right top front

        // Edges should have 4
        assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1,               32), 4);
        assert_surrounding((CHUNK_WIDTH - 1,                0,               32), 4);
        assert_surrounding((CHUNK_WIDTH - 1,               32, CHUNK_LENGTH - 1), 4);
        assert_surrounding((CHUNK_WIDTH - 1, CHUNK_HEIGHT - 1,               32), 4);
        assert_surrounding((CHUNK_WIDTH - 1,               32,                0), 4);
        assert_surrounding((CHUNK_WIDTH - 1,                0,               32), 4);
        assert_surrounding((              0, CHUNK_HEIGHT - 1,               32), 4);
        assert_surrounding((              0,                0,               32), 4);
        assert_surrounding((              0,               32, CHUNK_LENGTH - 1), 4);
        assert_surrounding((              0, CHUNK_HEIGHT - 1,               32), 4);
        assert_surrounding((              0,               32,                0), 4);
        assert_surrounding((              0,                0,               32), 4);

        // Inner sides should have 5
        assert_surrounding((CHUNK_WIDTH - 1, 32, 32), 5);
        assert_surrounding((32, CHUNK_HEIGHT - 1, 32), 5);
        assert_surrounding((32, 32, CHUNK_LENGTH - 1), 5);

        // Interiors should have 6
        assert_surrounding((32, 32, 32), 6);
    }
}

