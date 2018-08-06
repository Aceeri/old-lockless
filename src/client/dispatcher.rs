use std::borrow::Borrow;

use amethyst::assets::Processor;
use amethyst::audio::AudioFormat;
use amethyst::audio::Source;
use amethyst::core::Parent;
use amethyst::input::{Bindings, InputSystem};
use amethyst::prelude::*;
use amethyst::renderer::{
    AmbientColor, DrawPbm, DrawPbmSeparate, Pipeline, PosNormTangTex, RenderSystem, Rgba, Stage,
    TextureFormat,
};
use amethyst::ui::{
    UiEvent, DrawUi, FontAsset, FontFormat, ResizeSystem, UiButtonSystem, UiKeyboardSystem, UiLoaderSystem,
    UiMouseSystem, UiTransformSystem,
};
use amethyst::utils::fps_counter::FPSCounterSystem;

use shred::RunNow;
use shrev::EventChannel;
use specs::prelude::*;

use rayon::ThreadPool;

use error::Error;

const AMBIENT_LIGHT_COLOUR: Rgba = Rgba(0.002, 0.002, 0.002, 1.0); // near-black

pub fn dispatcher<P: 'static + Borrow<ThreadPool>>(
    world: &mut World,
    _p: P,
) -> Result<Box<for<'a> RunNow<'a>>, Error> {
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawPbm::<PosNormTangTex>::new())
            //.with_pass(DrawPbmSeparate::new().with_vertex_skinning())
            .with_pass(DrawUi::new()),
    );

    let display_config = ::amethyst::renderer::DisplayConfig {
        title: "Lockless".to_string(),
        fullscreen: false,
        dimensions: None,
        min_dimensions: None,
        max_dimensions: None,
        vsync: false,
        multisampling: 1,
        visibility: true,
    };

    world.add_resource(AmbientColor(AMBIENT_LIGHT_COLOUR));
    let render_system =
        RenderSystem::build(pipe, Some(display_config)).map_err(|e| Error::Amethyst(e.into()))?;

    let key_bindings_path = format!("{}/resources/input.ron", env!("CARGO_MANIFEST_DIR"));

    let mut dispatcher = DispatcherBuilder::new()
        .with(
            ::specs_hierarchy::HierarchySystem::<Parent>::new(),
            "hierarchy_system_parent",
            &[],
        )
        .with(
            InputSystem::<String, String>::new(Some(Bindings::load(key_bindings_path))),
            "input_system",
            &[],
        )
        .with(
            ::amethyst::controls::MouseFocusUpdateSystem::new(),
            "mouse_focus",
            &[],
        )
        .with(
            ::systems::controller::FlyCameraSystem::new(15.0, 0.3, 0.3),
            "fly_system",
            &["input_system", "mouse_focus"],
        )
        .with(
            ::systems::controller::FollowCameraSystem {
                hover_distance: 30.0,
                follow_speed: 5.0,
                rotation_speed: 8.0,
            },
            "follow_camera_system",
            &[],
        )
        .with(
            ::amethyst::core::transform::TransformSystem::new(),
            "transform_system",
            &["hierarchy_system_parent"],
        )
        .with(
            ::systems::physics::HandleRemovalSystem3d::new(),
            "handle_removal_system_3d",
            &[],
        )
        .with(
            ::systems::physics::PhysicsStep3d::new(),
            "physics_step_3d",
            &["handle_removal_system_3d"],
        )
        .with(
            ::systems::physics::SyncBodySystem3d::new(),
            "sync_body_3d",
            &["physics_step_3d"],
        )
        .with(
            UiLoaderSystem::<AudioFormat, TextureFormat, FontFormat>::default(),
            "ui_loader",
            &[],
        )
        .with(
            Processor::<FontAsset>::new(),
            "font_processor",
            &["ui_loader"],
        )
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with(
            UiKeyboardSystem::new(),
            "ui_keyboard_system",
            &["font_processor"],
        )
        .with(ResizeSystem::new(), "ui_resize_system", &[])
        .with(
            UiTransformSystem::default(),
            "ui_transform",
            &["transform_system"],
        )
        .with(
            UiMouseSystem::<String, String>::new(),
            "ui_mouse_system",
            &["ui_transform"],
        )
        .with(
            UiButtonSystem::new(),
            "ui_button_system",
            &["ui_mouse_system"],
        )
        .with(FPSCounterSystem, "fps_counter_system", &[])
        .with(
            ::systems::utils::fps_counter::FPSRenderSystem,
            "fps_render_system",
            &[],
        )
        .with(
            ::systems::objects::lights::LightFlickeringSystem,
            "light_flicker",
            &[],
        )
        .with(::systems::controller::CameraResizeSystem::default(), "camera_resize_system", &[])
        .with(UiEventHandlerSystem::new(), "ui_event_handler_system", &[])
        .with_thread_local(render_system)
        .build();

    dispatcher.setup(&mut world.res);

    info!("client dispatcher created");
    Ok(Box::new(dispatcher))
}

/// This shows how to handle UI events.
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        UiEventHandlerSystem { reader_id: None }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, mut events: Self::SystemData) {
        if self.reader_id.is_none() {
            self.reader_id = Some(events.register_reader());
        }
        for ev in events.read(self.reader_id.as_mut().unwrap()) {
            info!("You just interacted with a ui element: {:?}", ev);
        }
    }
}
