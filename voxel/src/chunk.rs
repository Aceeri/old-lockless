
use crate::block::{Block, BlockSize};

pub const CHUNK_HEIGHT: usize = 64; // Y
pub const CHUNK_WIDTH: usize = 64; // X
pub const CHUNK_LENGTH: usize = 64; // Z
pub const CHUNK_SIZE: usize = CHUNK_HEIGHT * CHUNK_WIDTH * CHUNK_LENGTH;

pub const EMPTY_BLOCK: Block = Block::hard_create(0);

#[derive(Debug, Copy, Clone)]
pub struct LocalBlockPosition {
    y: usize,
    x: usize,
    z: usize,
}

impl LocalBlockPosition {
    pub fn new(x: usize, y: usize, z: usize) -> Option<LocalBlockPosition> {
        if x <= CHUNK_WIDTH && y <= CHUNK_HEIGHT && z <= CHUNK_LENGTH {
            Some(LocalBlockPosition { x, y, z })
        } else {
            None
        }
    }
}

pub struct Chunk {
    // Stored in yxz order for caching (we will probably hit horizontal axis together).
    blocks: Box<[Block]>,
}

impl Chunk {
    pub fn block_index(&self, position: &LocalBlockPosition) -> usize {
        (position.y * CHUNK_HEIGHT * CHUNK_WIDTH) + (position.x * CHUNK_WIDTH) + position.z
    }

    pub fn block(&self, position: &LocalBlockPosition) -> Block {
        self.blocks[self.block_index(position)]
    }

    pub fn set_block(&mut self, position: &LocalBlockPosition, block: Block) {
        self.blocks[self.block_index(position)] = block;
    }

    pub fn empty_chunk() -> Chunk {
        Chunk {
            blocks: vec![EMPTY_BLOCK; CHUNK_SIZE].into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chunk::{Chunk, LocalBlockPosition, CHUNK_HEIGHT, CHUNK_WIDTH, CHUNK_LENGTH, CHUNK_SIZE};
    use crate::block::{Block, BlockSize, MAX_BLOCK_ID};

    fn test_chunk() -> Chunk {
        let data = (0..CHUNK_SIZE)
            .map(|position| {
                Block::hard_create((position % MAX_BLOCK_ID) as BlockSize)
            })
            .collect::<Vec<Block>>();

        return Chunk {
            blocks: data.into_boxed_slice(),
        }
    }

    #[test]
    fn block_position() {
        let chunk = test_chunk();
        println!("chunk size: {}", CHUNK_SIZE);
        let mut current = 0;
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_LENGTH {
                    let position = LocalBlockPosition::new(x, y, z).unwrap();
                    let block = chunk.block(&position);
                    let checking = Block::hard_create(current);
                    assert_eq!(checking, block);
                    current = ((current as usize % MAX_BLOCK_ID) + 1) as BlockSize;
                }
            }
        }
    }
}

