
use amethyst::renderer::{Event, KeyboardInput, WindowEvent, VirtualKeyCode};

use machinae::{State, Trans};

use world::application::GameData;

#[derive(Clone, Debug)]
pub enum GameState {
    Running,

    Menu,
    Loading,
}

impl GameState {
    pub fn new() -> Self {
        GameState::Loading
    }
}

impl<'a> State<&'a mut GameData, (), Event> for GameState {
    fn start(&mut self, args: &mut GameData) -> Result<Trans<Self>, ()> {
        println!("{:?} starting", self);
        match *self {
            GameState::Loading => Ok(Trans::Switch(GameState::Menu)),
            GameState::Menu => Ok(Trans::Switch(GameState::Running)),
            GameState::Running => Ok(Trans::None),
        }
    }

    fn update(&mut self, data: &mut GameData) -> Result<Trans<Self>, ()> {
        data.dispatcher.run_now(&mut data.world.res);
        data.world.maintain();

        //println!("step {:?}", self);

        Ok(Trans::None)
    }

    fn event(&mut self, data: &mut GameData, event: Event) -> Result<Trans<Self>, ()> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Ok(Trans::Quit),
                WindowEvent::Destroyed |
                WindowEvent::CloseRequested => Ok(Trans::Quit),
                _ => Ok(Trans::None),
            },
            _ => Ok(Trans::None),
        }
    }
}
