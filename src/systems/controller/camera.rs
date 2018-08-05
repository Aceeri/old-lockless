use cgmath::{Deg, Euler, Quaternion, Rad, Vector3, Zero};
use specs::prelude::*;

use amethyst::controls::{FlyControlTag, WindowFocus};
use amethyst::core::{timing::Time, Transform};
use amethyst::input::InputHandler;
use amethyst::winit::{DeviceEvent, Event};
use shrev::EventChannel;

/// The system that manages the view rotation.
/// Controlled by the mouse.
pub struct FlyCameraSystem {
    sensitivity_x: f32,
    sensitivity_y: f32,
    event_reader: Option<ReaderId<Event>>,

    /// The movement speed of the movement in units per second.
    speed: f32,
    /// The name of the input axis to locally move in the x coordinates.
    right_input_axis: String,
    /// The name of the input axis to locally move in the y coordinates.
    up_input_axis: String,
    /// The name of the input axis to locally move in the z coordinates.
    forward_input_axis: String,
}

impl FlyCameraSystem {
    pub fn new(speed: f32, sensitivity_x: f32, sensitivity_y: f32) -> Self {
        FlyCameraSystem {
            sensitivity_x,
            sensitivity_y,
            speed,

            event_reader: None,
            right_input_axis: "move_x".to_owned(),
            up_input_axis: "move_z".to_owned(),
            forward_input_axis: "move_y".to_owned(),
        }
    }

    fn get_axis(name: &str, input: &InputHandler<String, String>) -> f32 {
        input.axis_value(name).unwrap_or(0.0) as f32
    }
}

impl<'a> System<'a> for FlyCameraSystem {
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, FlyControlTag>,
        Read<'a, WindowFocus>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (events, mut transform, tag, focus, input, time): Self::SystemData) {
        let focused = focus.is_focused;

        for event in events.read(&mut self.event_reader.as_mut().unwrap()) {
            if focused {
                match *event {
                    Event::DeviceEvent { ref event, .. } => match *event {
                        DeviceEvent::MouseMotion { delta: (x, y) } => {
                            for (transform, _) in (&mut transform, &tag).join() {
                                transform.roll_global(Deg((1.0) * x as f32 * self.sensitivity_x));

                                let before_pitch = transform.rotation.clone();
                                let local_pitch = Deg((-1.0) * y as f32 * self.sensitivity_y);
                                transform.pitch_local(local_pitch);

                                let rot_w = transform.rotation.s;
                                let rot_x = transform.rotation.v.x;
                                let rot_y = transform.rotation.v.y;
                                let rot_z = transform.rotation.v.z;
                                let pitch: Deg<f32> = Rad(f32::atan2(
                                    2.0 * rot_x * rot_w + 2.0 * rot_y * rot_z,
                                    1.0 - 2.0 * rot_x * rot_x - 2.0 * rot_z * rot_z,
                                )).into();

                                if pitch < Deg::zero() {
                                    transform.rotation = before_pitch;
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        if focused {
            let x = FlyCameraSystem::get_axis(&self.right_input_axis, &input);
            let y = FlyCameraSystem::get_axis(&self.up_input_axis, &input);
            let z = FlyCameraSystem::get_axis(&self.forward_input_axis, &input);

            let dir = Vector3::new(x, y, z);

            for (transform, _) in (&mut transform, &tag).join() {
                transform.move_along_local(dir, time.delta_seconds() * self.speed);
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}
