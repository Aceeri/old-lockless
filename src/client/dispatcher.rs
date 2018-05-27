
use std::borrow::Borrow;

use amethyst::renderer::{AmbientColor, Rgba, Light, DrawFlat, DrawShaded, Pipeline, Stage, PosNormTex, RenderSystem, PointLight};
use amethyst::core::{Parent};
use amethyst::core::transform::TransformSystem;

use specs::prelude::*;
use shred::{ParSeq, RunNow, RunWithPool};

use smallvec::SmallVec;

use rayon::ThreadPool;

use ::error::Error;

const SPHERE_COLOUR: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; // blue
const AMBIENT_LIGHT_COLOUR: Rgba = Rgba(0.002, 0.002, 0.002, 1.0); // near-black
const POINT_LIGHT_COLOUR: Rgba = Rgba(1.0, 1.0, 1.0, 1.0); // white
const BACKGROUND_COLOUR: [f32; 4] = [0.0, 0.0, 0.0, 0.0]; // black
const LIGHT_POSITION: [f32; 3] = [2.0, 2.0, 6.0];
const LIGHT_RADIUS: f32 = 500.0;
const LIGHT_INTENSITY: f32 = 10.0;

pub type ThreadLocal<'a> = SmallVec<[Box<for<'b> RunNow<'b> + 'a>; 4]>;
pub struct ClientDispatcher<'a, P, R> {
    par_seq: ParSeq<P, R>, 
    thread_local: ThreadLocal<'a>,
}

impl<'a, 'b, P, R> RunNow<'a> for ClientDispatcher<'b, P, R>
where
    P: Borrow<ThreadPool>,
    R: for<'c> RunWithPool<'c>,
{
    fn run_now(&mut self, res: &Resources) {
        ParSeq::dispatch(&mut self.par_seq, res);

        for sys in &mut self.thread_local {
            sys.run_now(res);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        ParSeq::setup(&mut self.par_seq, res);

        for sys in &mut self.thread_local {
            sys.setup(res);
        }
    }
}


pub fn dispatcher<P: 'static + Borrow<ThreadPool>>(world: &mut World, p: P) -> Result<Box<for<'a> RunNow<'a>>, Error> {
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new())
    );

    let par_seq = ParSeq::new(
        //par![
            seq![
                ::specs_hierarchy::HierarchySystem::<Parent>::new(),
                TransformSystem::new(),
                ::systems::physics::HandleRemovalSystem3d::new(),
                ::systems::physics::PhysicsStep3d::new(),
                ::systems::physics::SyncBodySystem3d::new(),
            ],
        //],
        p,
    );

    let display_config = ::amethyst::renderer::DisplayConfig {
        title: "Lockless".to_string(),
        fullscreen: false,
        dimensions: None,
        min_dimensions: None,
        max_dimensions: None,
        vsync: true,
        multisampling: 1,
        visibility: true,
    };


    world.add_resource(AmbientColor(AMBIENT_LIGHT_COLOUR));

    let light: Light = PointLight {
        center: LIGHT_POSITION.into(),
        radius: LIGHT_RADIUS,
        intensity: LIGHT_INTENSITY,
        color: POINT_LIGHT_COLOUR,
        ..Default::default()
    }.into();

    world.register::<Light>();
    world.create_entity().with(light).build();

    let render_system = RenderSystem::build(pipe, Some(display_config)).map_err(|e| Error::Amethyst(e.into()))?;
    let mut thread_local = ThreadLocal::new();
    thread_local.push(Box::new(render_system));

    let mut client_dispatcher = ClientDispatcher {
        par_seq,
        thread_local,
    };
    client_dispatcher.setup(&mut world.res);

    println!("client dispatcher created");
    Ok(Box::new(client_dispatcher))
}
