use amethyst::core::{GlobalTransform, Time, Transform};
use amethyst::input::InputHandler;
use amethyst::winit::VirtualKeyCode;
use cgmath::{Point3, Vector3, Transform as CgTransform};

use specs::prelude::*;

pub struct FollowCameraTag {
    pub entity: Entity,
}

impl Component for FollowCameraTag {
    type Storage = DenseVecStorage<Self>;
}

pub struct FollowCameraSystem {
    pub hover_distance: f32,
    pub follow_speed: f32,
    pub rotation_speed: f32,
}

impl<'a> System<'a> for FollowCameraSystem {
    type SystemData = (
        ReadStorage<'a, GlobalTransform>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, FollowCameraTag>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (globals, mut transforms, tags, input, time): Self::SystemData) {
        if input.key_is_down(VirtualKeyCode::R) {
            for (mut restricted, tag) in (&mut transforms.restrict_mut(), &tags).join() {
                let target = globals.get(tag.entity);

                if let Some(target) = target {
                    let mut camera_transform = restricted.get_mut_unchecked();
                    let target_vector = Vector3::new(target.0.w.x, target.0.w.y, target.0.w.z + self.hover_distance);
                    camera_transform.translation = ::systems::utils::vector_interpolate(
                        camera_transform.translation,
                        target_vector,
                        time.delta_seconds() * self.follow_speed,
                    );

                    camera_transform.rotation = camera_transform.rotation.nlerp(<Transform as CgTransform<Point3<f32>>>::look_at(
                        Point3::new(0.0, 0.0, 0.0),
                        Point3::new(0.0, 0.0, 1.0),
                        Vector3::new(0.0, 1.0, 0.0),
                    ).rotation, time.delta_seconds() * self.rotation_speed);
                }
            }
        }
    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
