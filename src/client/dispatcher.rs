
use std::borrow::Borrow;

use amethyst::core::{Parent};
use amethyst::renderer::{
    AmbientColor, DrawShaded, DrawShadedSeparate, Pipeline, PosNormTex, RenderSystem,
    Rgba, Stage, TextureFormat
};
use amethyst::input::{Bindings, InputSystem};
use amethyst::audio::AudioFormat;
use amethyst::ui::{DrawUi, FontAsset, FontFormat, UiButtonSystem, UiMouseSystem, UiTransformSystem, UiLoaderSystem, UiKeyboardSystem, ResizeSystem};
use amethyst::prelude::*; 
use amethyst::utils::fps_counter::FPSCounterSystem;
use amethyst::assets::Processor;
use amethyst::audio::Source;

//use shred::{ParSeq, RunNow, RunWithPool};
use shred::RunNow;
use specs::prelude::*;

//use smallvec::SmallVec;

use rayon::ThreadPool;

use error::Error;

const AMBIENT_LIGHT_COLOUR: Rgba = Rgba(0.002, 0.002, 0.002, 1.0); // near-black

//pub type ThreadLocal<'a> = SmallVec<[Box<for<'b> RunNow<'b> + 'a>; 4]>;
//pub struct ClientDispatcher<'a, P, R> {
//par_seq: ParSeq<P, R>,
//thread_local: ThreadLocal<'a>,
//}

//impl<'a, 'b, P, R> RunNow<'a> for ClientDispatcher<'b, P, R>
//where
//P: Borrow<ThreadPool>,
//R: for<'c> RunWithPool<'c>,
//{
//fn run_now(&mut self, res: &Resources) {
//ParSeq::dispatch(&mut self.par_seq, res);

//for sys in &mut self.thread_local {
//sys.run_now(res);
//}
//}

//fn setup(&mut self, res: &mut Resources) {
//ParSeq::setup(&mut self.par_seq, res);

//for sys in &mut self.thread_local {
//sys.setup(res);
//}
//}
//}

pub fn dispatcher<P: 'static + Borrow<ThreadPool>>(
    world: &mut World,
    _p: P,
) -> Result<Box<for<'a> RunNow<'a>>, Error> {
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new())
            .with_pass(DrawShadedSeparate::new().with_vertex_skinning())
            .with_pass(DrawUi::new())
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

    let key_bindings_path = format!(
        "{}/resources/input.ron",
        env!("CARGO_MANIFEST_DIR")
    );

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
            ::systems::controller::FlyCameraSystem::new(5.0, 0.3, 0.3),
            "fly_system",
            &["input_system", "mouse_focus"],
        )
        .with(
            ::systems::controller::FollowCameraSystem{
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
        .with(::systems::utils::fps_counter::FPSRenderSystem, "fps_render_system", &[])
        .with_thread_local(render_system)
        .build();

    //let mut thread_local = ThreadLocal::new();
    //thread_local.push(Box::new(render_system));

    //let mut client_dispatcher = ClientDispatcher {
    //par_seq,
    //thread_local,
    //};
    //client_dispatcher.setup(&mut world.res);
    //dispatcher_builder.add_thread_local(render_system);

    //let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut world.res);

    println!("client dispatcher created");
    Ok(Box::new(dispatcher))
}
