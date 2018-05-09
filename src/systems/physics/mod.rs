
use nphysics3d::object::{BodyHandle, ColliderHandle};
use specs::prelude::*;
use specs::storage::DenseVecStorage;

pub use self::sync::PhysicsSyncSystem;

mod sync;

pub struct PhysicsBody {
    handle: BodyHandle,
}

impl Component for PhysicsBody {
    type Storage = DenseVecStorage<Self>;
}

pub struct PhysicsCollider {
    handle: ColliderHandle,
}

impl Component for PhysicsCollider {
    type Storage = DenseVecStorage<Self>;
}


