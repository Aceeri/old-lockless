
use amethyst::core::GlobalTransform;

use specs::prelude::*;

use shred::Fetch;

use super::{Body, PhysicsWorld3d};

pub struct SyncBodySystem; 

impl SyncBodySystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for SyncBodySystem {
    type SystemData = (
        Read<'a, PhysicsWorld3d>,
        ReadStorage<'a, Body>,
        WriteStorage<'a, GlobalTransform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (physics_world, rigid_bodies, mut transforms) = data;

        for (body, mut transform_restricted) in (
            &rigid_bodies, &mut transforms.restrict_mut()
        ).join() {
            let body = physics_world.body(body.handle);
            if body.is_ground() || !body.is_active() || body.is_static() {
                continue;
            }

            let position = match body {
                ::nphysics3d::object::Body::RigidBody(rigid_body) => rigid_body.position(),
                ::nphysics3d::object::Body::Multibody(multi_body) => {
                    match multi_body.links().next() {
                        Some(link) => link.position(),
                        None => continue
                    }
                }
                ::nphysics3d::object::Body::Ground(ground) => ground.position(),
            };

            let matrix = position.to_homogeneous();
            let fixed: [[f32; 4]; 4] = matrix.into();
            transform_restricted.get_mut_unchecked().0 = fixed.into();
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res); 
    }
}
