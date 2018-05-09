
extern crate amethyst;
extern crate machinae;
extern crate nphysics3d;
extern crate specs;
extern crate shrev;
extern crate rayon;
extern crate smallvec;

#[macro_use]
extern crate shred;

pub use world::application::Application;

pub mod systems;
pub mod world;

pub mod client;
