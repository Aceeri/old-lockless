
use std::ops::{Deref, DerefMut};

use specs::prelude::*;
use specs::storage::DenseVecStorage;

pub use self::sync::SyncBodySystem;
pub use self::components::*;

mod sync;
mod components;

type nphysicsWorld3d = ::nphysics3d::world::World<f32>;
pub struct PhysicsWorld3d {
    world: nphysicsWorld3d,
}

impl Default for PhysicsWorld3d {
    fn default() -> Self {
        PhysicsWorld3d {
            world: nphysicsWorld3d::new()
        }
    }
}

impl Deref for PhysicsWorld3d {
    type Target = nphysicsWorld3d;
    fn deref(&self) -> &nphysicsWorld3d {
        &self.world
    }
}

impl DerefMut for PhysicsWorld3d {
    fn deref_mut(&mut self) -> &mut nphysicsWorld3d {
        &mut self.world
    }
}
