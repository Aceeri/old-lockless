
pub type BlockSize = u16;
pub const MAX_BLOCK_ID: usize = 2 << (::std::mem::size_of::<BlockSize>() * 8 - 1);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Block(BlockSize);

impl Block {
    pub(crate) const fn hard_create(block: u16) -> Block {
        Block(block)
    }

    pub fn blocktype(&self) -> u16 {
        self.0 & 0b1111_1111_1111_0000
    }

    pub fn subtype(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_1111) as u8
    }
}
