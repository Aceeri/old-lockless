
pub type BlockSize = u16;
pub const MAX_BLOCK_ID: usize = 2 << (::std::mem::size_of::<BlockSize>() * 8 - 1);

pub const EMPTY_BLOCK: Block = Block::hard_create(0);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Block(BlockSize);

impl Block {
    pub const fn hard_create(block: u16) -> Block {
        Block(block)
    }

    pub fn id(&self) -> u16 {
        self.0
    }
}

