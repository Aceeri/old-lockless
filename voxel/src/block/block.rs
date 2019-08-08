
use crate::block::BlockDeclaration;

pub type BlockSize = u16;
pub const BLOCK_TYPE_MASK: BlockSize = 0b1111_1111_1111_0000;
pub const BLOCK_SUBTYPE_MASK: BlockSize = 0b0000_0000_0000_1111;
pub const MAX_BLOCK_ID: usize = 2 << (::std::mem::size_of::<BlockSize>() * 8 - 1);
pub const MAX_BLOCK_TYPE: usize = 2 << (BLOCK_TYPE_MASK.count_ones() - 1);
pub const MAX_BLOCK_SUBTYPE: usize = 2 << (BLOCK_SUBTYPE_MASK.count_ones() - 1);

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

    pub fn block_type(&self) -> u16 {
        self.id() & BLOCK_TYPE_MASK
    }

    pub fn sub_type(&self) -> u16 {
        self.id() & BLOCK_SUBTYPE_MASK
    }
}

