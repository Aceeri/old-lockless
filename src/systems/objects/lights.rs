use amethyst::core::Time;
use amethyst::renderer::{Light};
use specs::prelude::*;

use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct FlickerLight {
    pub intensity: f32,

    target_intensity: f32,
    current_flicker: f32,
    accumulation: f32,
    next_flicker: f32,
}

impl FlickerLight {
    pub fn new(intensity: f32) -> Self {
        FlickerLight {
            intensity,
            ..Default::default()
        }
    }
}

impl Component for FlickerLight {
    type Storage = DenseVecStorage<Self>;
}

pub struct LightFlickeringSystem;

impl<'a> System<'a> for LightFlickeringSystem {
    type SystemData = (
        WriteStorage<'a, FlickerLight>,
        WriteStorage<'a, Light>,
        Read<'a, Time>,
    );
    fn run(&mut self, (mut flickers, mut lights, time): Self::SystemData) {
        for (mut light, mut flicker) in (&mut lights, &mut flickers).join() {
            match light {
                Light::Point(point_light) => {
                    point_light.intensity = ::systems::utils::linear_interpolate(point_light.intensity, flicker.target_intensity, time.delta_seconds() * 5.0);

                    flicker.accumulation += time.delta_seconds();
                    let mut rng = thread_rng();

                    if flicker.current_flicker <= 0.0 && flicker.accumulation >= flicker.next_flicker
                    {
                        flicker.current_flicker += rng.gen_range(0.1, 0.3);
                        flicker.target_intensity = flicker.intensity - flicker.intensity * rng.gen_range(0.04, 0.7);
                        flicker.next_flicker = rng.gen_range(0.4, 0.9);
                    }

                    if flicker.current_flicker > 0.0 {
                        flicker.current_flicker -= time.delta_seconds();
                    }

                    if flicker.current_flicker <= 0.0 {
                        flicker.target_intensity = flicker.intensity;
                    }
                }
                _ => {}
            }
        }
    }
}
