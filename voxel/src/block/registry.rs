//! Basic structure of a registry file:
//!
//! ```ignore
//! {
//!     // Block ids are stored in a map instead of array for easier manual lookup.
//!
//!     0: {
//!         group: "Empty",
//!         name: "Air", 
//!         color: (0, 0, 0), // RGB (0-255, 0-255, 0-255)
//!         transparency: 255, // Transparency of block (0-255)
//!         collidable: 0, // Whether the player moves through the block // (0 = not collidable, 255 = fully stable) 
//!         hardness: 255, // Destructability (255 = indestructible)
//!     },
//! }
//! ```

use std::path::Path;
use std::collections::HashMap;
use std::io;

use crate::block::{Block, BlockSize, MAX_BLOCK_ID};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDeclaration {
    group: String,
    name: String,
    color: (u8, u8, u8),
    transparency: u8,
    //collidable: u8,
    //hardness: u8,
}

impl BlockDeclaration {
    pub fn visible(&self) -> bool {
        self.transparency != 255
    }

    pub fn opaque(&self) -> bool {
        self.transparency == 0
    }

    pub fn transparency(&self) -> u8 {
        self.transparency
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlockRegistryFile(HashMap<usize, BlockDeclaration>);

#[derive(Debug)]
pub struct FailedDeclaration {
    id: usize,
    subtype: Option<usize>,
    reason: String,
}

impl BlockRegistryFile {
    pub fn into_registry(&self, registry: &mut BlockRegistry) -> Vec<FailedDeclaration> {
        let mut failures = Vec::new();
        for (id, declaration) in &self.0 {
            if *id > MAX_BLOCK_ID {
                failures.push(FailedDeclaration {
                    id: *id,
                    subtype: None,
                    reason: format!("id {} is larger than max block type {}", id, MAX_BLOCK_ID),
                });

                continue;
            }

            registry.set_declaration(*id as u16, Some(declaration.clone()));
        }
        failures
    }
}

#[derive(Clone)]
pub struct BlockRegistry {
    registry: Vec<Option<BlockDeclaration>>,
    groups: HashMap<String, Vec<usize>>,
}

#[derive(Debug)]
pub enum RegistryError {
    JSON(serde_json::Error),
    IO(io::Error)
}

impl BlockRegistry {
    pub fn empty() -> BlockRegistry {
        let registry = BlockRegistry {
            registry: vec![None; MAX_BLOCK_ID],
            groups: HashMap::new(),
        };

        registry
    }

    pub fn from_reader<R: io::Read>(reader: R) -> Result<(BlockRegistry, Vec<FailedDeclaration>), RegistryError> {
        let registry_file: BlockRegistryFile = serde_json::from_reader(reader)
            .map_err(|e| RegistryError::JSON(e))?;

        let mut registry = BlockRegistry::empty();
        let failures = registry_file.into_registry(&mut registry);
        Ok((registry, failures))
    }

    pub fn from_str(string: &str) -> Result<(BlockRegistry, Vec<FailedDeclaration>), RegistryError> {
        let registry_file: BlockRegistryFile = serde_json::from_str(string)
            .map_err(|e| RegistryError::JSON(e))?;

        let mut registry = BlockRegistry::empty();
        let failures = registry_file.into_registry(&mut registry);
        Ok((registry, failures))
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<(BlockRegistry, Vec<FailedDeclaration>), RegistryError> {
        let file = std::fs::File::open(path)
            .map_err(|e| RegistryError::IO(e))?;

        BlockRegistry::from_reader(file)
    }

    #[inline]
    pub fn declaration(&self, block: Block) -> &Option<BlockDeclaration> {
        &self.registry[block.id() as usize]
    }

    #[inline]
    pub fn set_declaration(&mut self, index: BlockSize, declaration: Option<BlockDeclaration>) {
        info!(util::LOG, "setting block {}: {:?}", index, declaration);
        self.registry[index as usize] = declaration;
    }

    pub fn blocks_in_group<S: AsRef<String>>(&self, group: S) -> Option<Vec<&BlockDeclaration>> {
        match self.groups.get(group.as_ref()) {
            Some(group) => {
                let mut grouped = Vec::new();
                for index in group {
                    if let Some(declaration) = self.registry[*index].as_ref() {
                        grouped.push(declaration);
                    }
                }
                Some(grouped)
            },
            None => None,
        }
    }
}

