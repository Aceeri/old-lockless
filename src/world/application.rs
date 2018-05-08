
use std::error::Error as StdError;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::marker::PhantomData;

use amethyst::assets::{Loader, Source};
use amethyst::core::{Stopwatch, Time};
use amethyst::core::frame_limiter::{FrameLimiter};
use amethyst::renderer::{Event, WindowEvent};
use amethyst::shrev::{EventChannel};

use rayon::{ThreadPool, ThreadPoolBuilder};

use specs::prelude::*;
use specs::common::Errors;

use shred::ParSeq;

use machinae::{StateMachineRef, State, Trans};

use world::state::GameState;

pub trait Dispatch {
    fn dispatch(&mut self, res: &Resources);
    fn setup(&mut self, res: &mut Resources);
}

impl<P, S> Dispatch for ParSeq<P, S> {
    fn dispatch(&mut self, res: &Resources) {
        self.dispatch(res);
    }

    fn setup(&mut self, res: &mut Resources) {
        self.setup(res);
    }
}

pub struct GameData<'a, S>{
    pub world: World,
    //pub dispatcher: ParSeq<&'a ThreadPool, S>,
    pub dispatcher: Box<Dispatch>,
    _phantom: PhantomData<(&'a (), S)>,
}

pub struct Application<'a, S> {
    pub state: StateMachineRef<GameData<'a, S>, (), Event, GameState>,
    pub data: GameData<'a, S>,
    events_reader_id: ReaderId<Event>,

    _phantom: PhantomData<(&'a (), S)>,
}

impl<'a, S> Application<'a, S> {
    pub fn new_client<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let state = GameState::new();
        let machine = StateMachineRef::new(state);
        let mut world = World::new();

        let thread_pool_builder = ThreadPoolBuilder::new();
        #[cfg(feature = "profiler")]
        let thread_pool_builder = thread_pool_builder.start_handler(|index| {
            register_thread_with_profiler(format!("thread_pool{}", index));
        });
        let pool = thread_pool_builder
            .build()
            .map(|p| Arc::new(p))
            .map_err(|err| err.description().to_string())?;
        let mut dispatcher = ::client::dispatcher(&mut world, pool.clone());
        world.add_resource(Loader::new(path.as_ref().to_owned(), pool.clone()));
        world.add_resource(pool);
        world.add_resource(EventChannel::<Event>::with_capacity(2000));
        world.add_resource(Errors::default());
        world.add_resource(FrameLimiter::default());
        world.add_resource(Stopwatch::default());
        world.add_resource(Time::default());

        dispatcher.setup(&mut world.res);
        let data = GameData {
            world: world,
            dispatcher: dispatcher,
            _phantom: PhantomData,
        };

        let events_reader_id = data.world.write_resource::<EventChannel<Event>>().register_reader();

        Ok(Application {
            state: machine,
            data,
            events_reader_id,
            _phantom: PhantomData,
        })
    }

    pub fn run(&mut self) {
        self.data.world.write_resource::<Stopwatch>().start();
        while self.state.running() {
            self.step();

            self.data.world.write_resource::<FrameLimiter>().wait();
            {
                let elapsed = self.data.world.read_resource::<Stopwatch>().elapsed();
                let mut time = self.data.world.write_resource::<Time>();
                time.increment_frame_number();
                time.set_delta_time(elapsed);
            }
            let mut stopwatch = self.data.world.write_resource::<Stopwatch>();
            stopwatch.stop();
            stopwatch.restart();
        }

        self.shutdown();
    }

    pub fn shutdown(&mut self) { }

    fn step(&mut self) {
        {
            let events = self.data.world
                .read_resource::<EventChannel<Event>>()
                .read(&mut self.events_reader_id)
                .cloned()
                .collect::<Vec<_>>();

            for event in events {
                self.state.event(&mut self.data, event.clone());
            }
        }

        {
            let do_fixed = {
                let time = self.data.world.write_resource::<Time>();
                time.last_fixed_update().elapsed() >= time.fixed_time()
            };

            if do_fixed {
                self.state.fixed_update(&mut self.data);
                self.data.world.write_resource::<Time>().finish_fixed_update();
            }

            self.state.update(&mut self.data);
        }

        // TODO: replace this with a more customizable method.
        // TODO: effectively, the user should have more control over error handling here
        // TODO: because right now the app will just exit in case of an error.
        self.data.world.write_resource::<Errors>().print_and_exit();
    }
}
