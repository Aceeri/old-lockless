
extern crate amethyst;
extern crate nphysics3d;
extern crate specs;
extern crate machinae;
extern crate rayon;

#[macro_use]
extern crate shred;

pub use world::application::Application;

pub mod systems;
pub mod world;

pub mod client;
