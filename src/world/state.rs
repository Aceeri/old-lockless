
use amethyst::renderer::{Projection, Material, MaterialDefaults, ActiveCamera, Camera, Event, KeyboardInput, PosNormTex, WindowEvent, VirtualKeyCode, Mesh};
use amethyst::assets::{Handle, Loader};
use amethyst::core::{Transform, GlobalTransform};

use machinae::{State, Trans};

use world::application::GameData;

use cgmath::{Array, EuclideanSpace, One, Quaternion, Vector3};

use ::error::Error;

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

impl<'a> State<&'a mut GameData, Error, Event> for GameState {
    fn start(&mut self, data: &mut GameData) -> Result<Trans<Self>, Error> {
        println!("{:?} starting", self);
        match *self {
            GameState::Loading => {
                use genmesh::{MapToVertices, Triangulate, Vertices};
                use genmesh::generators::SphereUV;
                let vertices = SphereUV::new(50, 50)
                    .vertex(|v| PosNormTex {
                        position: v.pos.into(),
                        normal: v.normal.into(),
                        tex_coord: [0.1, 0.1],
                    })
                    .triangulate()
                        .vertices()
                        .collect::<Vec<_>>();

                let mesh: Handle<Mesh> = data.world
                    .read_resource::<Loader>()
                    .load_from_data(vertices.into(), (), &data.world.read_resource());

                let albedo = data.world.read_resource::<Loader>().load_from_data(
                    [1.0, 0.0, 0.0, 1.0].into(),
                    (),
                    &data.world.read_resource(),
                );
                let material = Material {
                    albedo,
                    ..data.world.read_resource::<MaterialDefaults>().0.clone()
                };

                data.world
                    .create_entity()
                    .with(mesh)
                    .with(material)
                    .with(Transform {
                        translation: ::cgmath::Point3::new(-3., 0., -3.).to_vec(),
                        rotation: ::cgmath::Quaternion::<f32>::one(),
                        scale: ::cgmath::Vector3::from_value(1.),
                    })
                    .with(GlobalTransform::default())
                    .build();

                let camera_entity = data.world
                    .create_entity()
                    .with(Camera::standard_3d(500., 500.))
                    .with(Transform {
                        rotation: Quaternion::one(),
                        scale: Vector3::from_value(1.0),
                        translation: Vector3::new(0., 0., 5.0),
                    })
                    .with(GlobalTransform::default())
                    .build();

                data.world.add_resource(ActiveCamera {
                    entity: camera_entity,    
                });

                println!("{:?} switching to {:?}", self, GameState::Menu);
                Ok(Trans::Switch(GameState::Menu))
            }
            GameState::Menu => {
                println!("{:?} switching to {:?}", self, GameState::Running);
                Ok(Trans::Switch(GameState::Running))
            }
            GameState::Running => Ok(Trans::None),
        }
    }

    fn update(&mut self, data: &mut GameData) -> Result<Trans<Self>, Error> {
        data.dispatcher.run_now(&mut data.world.res);
        data.world.maintain();

        Ok(Trans::None)
    }

    fn event(&mut self, _data: &mut GameData, event: Event) -> Result<Trans<Self>, Error> {
        //println!("event: {:?}", event);
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

    fn resume(&mut self, _args: &mut GameData) {
        println!("{:?} resumed", self);
    }

    fn pause(&mut self, _args: &mut GameData) {
        println!("{:?} paused", self);
    }

    fn stop(&mut self, _args: &mut GameData) {
        println!("{:?} stopping", self);
    }
}
