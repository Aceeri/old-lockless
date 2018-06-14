use specs::prelude::*;

use amethyst::core::Time;

use super::PhysicsWorld3d;

pub struct PhysicsStep3d;

impl PhysicsStep3d {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for PhysicsStep3d {
    type SystemData = (Write<'a, PhysicsWorld3d>, Read<'a, Time>);
    fn run(&mut self, (mut world, time): Self::SystemData) {
        world.set_timestep(time.delta_seconds());
        world.step();
    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
