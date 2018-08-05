
use specs::prelude::*;
use amethyst::utils::fps_counter::FPSCounter;
use amethyst::ui::{UiText};
use amethyst::core::Time;

pub struct FPSTag;
impl Component for FPSTag {
    type Storage = DenseVecStorage<Self>;
}

pub struct FPSRenderSystem;

impl<'a> System<'a> for FPSRenderSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        ReadStorage<'a, FPSTag>,
        Read<'a, Time>,
        Read<'a, FPSCounter>,
    );
    fn run(&mut self, (mut text_uis, tags, time, fps_counter): Self::SystemData) {
        if time.frame_number() % 20 == 0 {
            let sampled_fps = fps_counter.sampled_fps();
            for (mut text, _) in (&mut text_uis, &tags).join() {
                text.text = format!("FPS: {:.*}", 0, sampled_fps);
            }
        }
    }
}
