
use std::path::Path;
use std::sync::Arc;

use amethyst::assets::{Loader};
use amethyst::core::{Stopwatch, Time};
use amethyst::core::frame_limiter::{FrameLimiter};
use amethyst::renderer::{Event};

use rayon::{ThreadPoolBuilder};

use specs::prelude::*;
use specs::common::Errors;

use shred::{RunNow};

use shrev::EventChannel;

use machinae::{StateMachineRef};

use world::state::GameState;

use ::error::Error;

pub struct GameData {
    pub world: World,
    pub dispatcher: Box<for<'a> RunNow<'a>>,
}

pub struct Application {
    pub state: StateMachineRef<GameData, Error, Event, GameState>,
    pub data: GameData,
    events_reader_id: ReaderId<Event>,
}

impl Application {
    pub fn new_client<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let thread_pool_builder = ThreadPoolBuilder::new();
        let pool = thread_pool_builder
            .build()
            .map(|p| Arc::new(p))
            .map_err(|err| Error::Rayon(err))?;

        let state = GameState::new();
        let machine = StateMachineRef::new(state);
        let mut world = World::new();
        world.add_resource(pool.clone());
        world.add_resource(Loader::new(path.as_ref().to_owned(), pool.clone()));
        world.add_resource(EventChannel::<Event>::with_capacity(2000));
        world.add_resource(Errors::default());
        world.add_resource(FrameLimiter::default());
        world.add_resource(Stopwatch::default());
        world.add_resource(Time::default());

        let mut dispatcher = ::client::dispatcher(&mut world, pool.clone())?;
        dispatcher.setup(&mut world.res);
        let data = GameData {
            world: world,
            dispatcher: dispatcher,
        };

        let events_reader_id = data.world.write_resource::<EventChannel<Event>>().register_reader();

        Ok(Application {
            state: machine,
            data,
            events_reader_id,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        println!("application starting");
        self.data.world.write_resource::<Stopwatch>().start();
        self.state.start(&mut self.data)?;
        while self.state.running() {
            self.step()?;

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
        Ok(())
    }

    pub fn shutdown(&mut self) { }

    fn step(&mut self) -> Result<(), Error> {
        {
            let events = self.data.world
                .read_resource::<EventChannel<Event>>()
                .read(&mut self.events_reader_id)
                .cloned()
                .collect::<Vec<_>>();

            for event in events {
                if !self.state.running() {
                    return Ok(())
                }

                self.state.event(&mut self.data, event.clone())?;
            }
        }

        {
            let do_fixed = {
                let time = self.data.world.write_resource::<Time>();
                time.last_fixed_update().elapsed() >= time.fixed_time()
            };

            if do_fixed {
                if !self.state.running() {
                    return Ok(())
                }

                self.state.fixed_update(&mut self.data)?;
                self.data.world.write_resource::<Time>().finish_fixed_update();
            }

            if !self.state.running() {
                return Ok(())
            }

            self.state.update(&mut self.data)?;
        }

        // TODO: replace this with a more customizable method.
        // TODO: effectively, the user should have more control over error handling here
        // TODO: because right now the app will just exit in case of an error.
        self.data.world.write_resource::<Errors>().print_and_exit();

        Ok(())
    }
}
