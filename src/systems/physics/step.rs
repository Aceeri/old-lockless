

pub struct PhysicsStepSystem {
    
}

impl<'a> System<'a> for PhysicsStepSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, PhysicsWorld3d>,
        ReadStorage<'a, PhysicsBody>,
        ReadStorage<'a, PhysicsCollider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, world, bodies, colliders) = data;
    }
}
