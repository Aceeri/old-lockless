
use specs::prelude::*;

pub struct PhysicsSyncSystem {

}

impl PhysicsSyncSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for PhysicsSyncSystem {
    type SystemData = ();
    fn run(&mut self, data: Self::SystemData) { }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res); 
    }
}
