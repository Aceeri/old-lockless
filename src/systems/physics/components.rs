
use nphysics3d::object::{BodyHandle, ColliderHandle, SensorHandle};
use specs::prelude::*;

pub struct Body {
    pub handle: BodyHandle,
}

impl Component for Body {
    type Storage = DenseVecStorage<Self>;
}

pub struct Collider {
    pub handle: ColliderHandle,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}

pub struct Sensor {
    pub handle: SensorHandle,
}

impl Component for Sensor {
    type Storage = DenseVecStorage<Self>;
}

