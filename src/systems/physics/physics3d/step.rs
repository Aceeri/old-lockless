
use specs::prelude::*;

use super::{PhysicsWorld3d};

pub struct PhysicsStep3d;

impl PhysicsStep3d {
    pub fn new() -> Self {
        Self { }
    }
}

impl<'a> System<'a> for PhysicsStep3d {
    type SystemData = (
        Write<'a, PhysicsWorld3d>,
    );
    fn run(&mut self, (mut world,): Self::SystemData) {
        //world.set_timestep();
        world.step();
    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
