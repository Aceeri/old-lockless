
use amethyst::renderer::{Event, WindowEvent};

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

impl<'a, 'b, S> State<&'a mut GameData<'b, S>, (), Event> for GameState {
    fn start(&mut self, args: &mut GameData<'b, S>) -> Result<Trans<Self>, ()> {
        match *self {
            GameState::Loading => Ok(Trans::Switch(GameState::Menu)),
            GameState::Menu => Ok(Trans::Switch(GameState::Running)),
            GameState::Running => Ok(Trans::None),
        }
    }

    fn update(&mut self, data: &mut GameData<'b, S>) -> Result<Trans<Self>, ()> {
        data.dispatcher.dispatch(&mut data.world.res);
        data.world.maintain();

        Ok(Trans::None)
    }
}
