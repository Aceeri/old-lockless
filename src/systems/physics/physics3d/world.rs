
use std::ops::{Deref, DerefMut};

type NphysicsWorld3d = ::nphysics3d::world::World<f32>;
pub struct PhysicsWorld3d {
    world: NphysicsWorld3d,
}

impl Default for PhysicsWorld3d {
    fn default() -> Self {
        PhysicsWorld3d {
            world: NphysicsWorld3d::new()
        }
    }
}

impl Deref for PhysicsWorld3d {
    type Target = NphysicsWorld3d;
    fn deref(&self) -> &NphysicsWorld3d {
        &self.world
    }
}

impl DerefMut for PhysicsWorld3d {
    fn deref_mut(&mut self) -> &mut NphysicsWorld3d {
        &mut self.world
    }
}
