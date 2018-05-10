
use std::error::Error;
use std::borrow::Borrow;

use amethyst::renderer::{DrawShaded, Pipeline, Stage, PosNormTex, RenderSystem};

use specs::prelude::*;
use shred::{ParSeq, RunNow, RunWithPool};

use smallvec::SmallVec;

use rayon::ThreadPool;

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


pub fn dispatcher<P: 'static + Borrow<ThreadPool>>(world: &mut World, p: P) -> Result<Box<for<'a> RunNow<'a>>, String> {
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new()),
    );

    let par_seq = ParSeq::new(
        par![
            ::systems::physics::SyncBodySystem::new(),
        ],
        p,
    );

    let render_system = RenderSystem::build(pipe, None).map_err(|e| e.description().to_string())?;
    let mut thread_local = ThreadLocal::new();
    thread_local.push(Box::new(render_system));

    Ok(Box::new(ClientDispatcher {
        par_seq,
        thread_local,
    }))
}
