
use nphysics3d::object::{BodyHandle, ColliderHandle, SensorHandle};
use specs::prelude::*;

pub struct Body3d {
    pub handle: BodyHandle,
}

impl Component for Body3d {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct Collider3d {
    pub handle: ColliderHandle,
}

impl Component for Collider3d {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct Sensor3d {
    pub handle: SensorHandle,
}

impl Component for Sensor3d {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

