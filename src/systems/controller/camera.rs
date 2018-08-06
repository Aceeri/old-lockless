
use amethyst::renderer::{ScreenDimensions, Camera, Projection};
use amethyst::winit::{Event, WindowEvent};
use cgmath::Deg;
use shrev::EventChannel;
use specs::prelude::*;

pub struct CameraResize {
    pub fov: f32,
}

impl Component for CameraResize {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct CameraResizeSystem {
    event_reader: Option<ReaderId<Event>>,
}

impl<'a> System<'a> for CameraResizeSystem {
    type SystemData = (
        WriteStorage<'a, Camera>,
        ReadStorage<'a, CameraResize>,
        Read<'a, EventChannel<Event>>,
    );
    fn run(&mut self, (mut cameras, tags, events): Self::SystemData) {
        for event in events.read(&mut self.event_reader.as_mut().unwrap()) {
            match *event {
                Event::WindowEvent { ref event, .. } => match *event {
                    WindowEvent::Resized(width, height) => {
                        for (mut camera, tag) in (&mut cameras, &tags).join() {
                            *camera = Camera::from(Projection::perspective(width as f32 / height as f32, Deg(tag.fov)));
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}
