//! Basic structure of a registry file:
//!
//! ```
//! {
//!     // Block ids are stored in a map instead of array for easier manual lookup.
//!
//!     0: {
//!         default: {
//!             name: "Air", 
//!             color: (0, 0, 0), // RGB (0-255, 0-255, 0-255)
//!             transparency: 255, // Transparency of block (0-255)
//!             collidable: 0, // Whether the player moves through the block 
//!             // (0 = not collidable, 255 = fully stable)
//!             hardness: 255, // Destructability (255 = indestructible)
//!         },
//!
//!         subtypes: {
//!             // Subtypes similarly are stored in map of id -> subtype.
//!
//!             0: { 
//!                 // These fields override the defaults of the block.
//!                 // All fields above are also valid here.
//!
//!                 name: "Air",
//!                 color: (0, 0, 0),
//!                 transparency: 255,
//!             }
//!         }
//!     },
//! }
//! ```

use std::path::Path;
use std::collections::HashMap;
use std::io;

use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::block::{Block, BlockSize, MAX_BLOCK_ID, MAX_BLOCK_TYPE, MAX_BLOCK_SUBTYPE, EMPTY_BLOCK};

macro_rules! block_declaration {
    ( $( $field:ident : $ty:ty ),* ) => {

        #[derive(Clone, Deserialize, Serialize)]
        pub struct BlockDeclaration {
            $( $field: $ty ),*
        }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct BlockDeclarationPartial {
            #[serde(default)]
            $( $field: Option<$ty> ),*
        }

        impl BlockDeclarationPartial {
            pub fn merge(&self, subtype: &BlockDeclarationPartial) -> BlockDeclarationPartial {
                let mut merged = self.clone();
                $(
                    if let Some(ref field) = subtype.$field {
                        merged.$field = Some(field.clone());
                    }
                )*
                merged
            }

            pub fn into_full(&self) -> Result<BlockDeclaration, Vec<(&'static str, &'static str)>> {
                let mut missing = Vec::new();
                $(
                    if let None = self.$field {
                        missing.push((stringify!($field), stringify!($ty)));
                    }
                )*

                if missing.len() > 0 {
                    return Err(missing);
                }

                Ok(BlockDeclaration {
                    $( $field: self.$field.clone().unwrap(), )*
                })
            }
        }
    };
}

block_declaration! {
    name: String,
    color: u16,
    transparency: u8
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BlockDeclarationParent {
    default: BlockDeclarationPartial,
    subtypes: HashMap<usize, BlockDeclarationPartial>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockRegistryFile(HashMap<usize, BlockDeclarationParent>);

#[derive(Debug)]
pub struct FailedDeclaration {
    id: usize,
    subtype: Option<usize>,
    reason: String,
}

impl BlockRegistryFile {
    pub fn into_registry(&self, registry: &mut BlockRegistry) -> Vec<FailedDeclaration> {
        let mut failures = Vec::new();
        for (id, parent) in &self.0 {
            if *id > MAX_BLOCK_TYPE {
                failures.push(FailedDeclaration {
                    id: *id,
                    subtype: None,
                    reason: format!("id {} is larger than max block type {}", id, MAX_BLOCK_TYPE),
                });

                continue;
            }

            for (subtype_id, partial) in &parent.subtypes {
                if *subtype_id > MAX_BLOCK_SUBTYPE {
                    failures.push(FailedDeclaration {
                        id: *id,
                        subtype: Some(*subtype_id),
                        reason: format!(
                            "subtype id {} (id {}) is larger than max block subtype {}", 
                            subtype_id, id, MAX_BLOCK_SUBTYPE
                        ),
                    });

                    continue;
                }

                let merged = parent.default.merge(partial);
                let declaration = match merged.into_full() {
                    Ok(declaration) => declaration,
                    Err(missing_fields) => {
                        failures.push(FailedDeclaration {
                            id: *id,
                            subtype: Some(*subtype_id),
                            reason: format!("missing fields: {:?}", missing_fields),
                        });

                        continue;
                    }
                };

                let index = MAX_BLOCK_TYPE * id; // 4096 * id.
                registry.set_declaration((index + subtype_id) as u16, Some(declaration));
            }
        }
        failures
    }
}

#[derive(Clone)]
pub struct BlockRegistry {
    registry: Vec<Option<BlockDeclaration>>,
}

pub enum RegistryError {
    JSON(serde_json::Error),
    IO(io::Error)
}

impl BlockRegistry {
    pub fn empty() -> BlockRegistry {
        let mut registry = BlockRegistry {
            registry: vec![None; MAX_BLOCK_ID],
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

    pub fn declaration(&self, block: Block) -> &Option<BlockDeclaration> {
        &self.registry[block.id() as usize]
    }

    pub fn set_declaration(&mut self, index: BlockSize, declaration: Option<BlockDeclaration>) {
        self.registry[index as usize] = declaration;
    }
}

