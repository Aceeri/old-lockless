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
    pub speed: f32,
}

fn linear_interpolate(from: f32, to: f32, amount: f32) -> f32 {
    (from * (1.0 - amount) + to * amount)
}

fn vector_interpolate(from: Vector3<f32>, to: Vector3<f32>, amount: f32) -> Vector3<f32> {
    Vector3::new(
        linear_interpolate(from.x, to.x, amount),
        linear_interpolate(from.y, to.y, amount),
        linear_interpolate(from.z, to.z, amount),
    )
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
                    camera_transform.translation = vector_interpolate(
                        camera_transform.translation,
                        target_vector,
                        time.delta_seconds() * self.speed,
                    );
                    println!(
                        "interpolating to {:?}",
                        target_vector,
                    );

                    camera_transform.rotation = <Transform as CgTransform<Point3<f32>>>::look_at(
                        Point3::new(0.0, 0.0, 0.0),
                        Point3::new(0.0, 0.0, 1.0),
                        Vector3::new(0.0, 1.0, 0.0),
                    ).rotation;
                }
            }
        }
    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }
}
