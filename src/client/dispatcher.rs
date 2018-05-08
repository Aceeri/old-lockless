
use std::sync::Arc;
use std::borrow::Borrow;

use rayon::{ThreadPool, ThreadPoolBuilder};

use specs::prelude::*;
use shred::ParSeq;

use world::application::Dispatch;

struct DummySys;
impl<'a> System<'a> for DummySys {
    type SystemData = ();
    fn run(&mut self, data: ()) { }
}

pub fn dispatcher<P: 'static + Borrow<ThreadPool>>(world: &mut World, p: P) -> Box<Dispatch> {
    Box::new(ParSeq::new(
        seq![
            DummySys,
            DummySys,
        ],
        p,
    )) as Box<Dispatch>
}
