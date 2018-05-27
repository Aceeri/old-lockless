use amethyst::assets::{Handle, Loader};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::renderer::{
    ActiveCamera, Camera, Event, KeyboardInput, MaterialDefaults, Mesh, PosNormTex, Projection,
    VirtualKeyCode, WindowEvent,
};

use machinae::{State, Trans};

use world::application::GameData;

use cgmath::{Array, EuclideanSpace, One};
use nalgebra::core::{Unit, Matrix3, Vector3};
use ncollide3d::shape::{Ball, Plane, ShapeHandle};
use nphysics3d::algebra::Inertia3;
use nphysics3d::math::{Inertia, Isometry, Point};
use nphysics3d::object::{BodyHandle, Material};

use error::Error;

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
                use genmesh::generators::SphereUV;
                use genmesh::{MapToVertices, Triangulate, Vertices};
                let vertices = SphereUV::new(50, 50)
                    .vertex(|v| PosNormTex {
                        position: v.pos.into(),
                        normal: v.normal.into(),
                        tex_coord: [0.1, 0.1],
                    })
                    .triangulate()
                    .vertices()
                    .collect::<Vec<_>>();

                let mesh: Handle<Mesh> = data.world.read_resource::<Loader>().load_from_data(
                    vertices.into(),
                    (),
                    &data.world.read_resource(),
                );

                let albedo = data.world.read_resource::<Loader>().load_from_data(
                    [1.0, 0.0, 0.0, 1.0].into(),
                    (),
                    &data.world.read_resource(),
                );
                let material = ::amethyst::renderer::Material {
                    albedo,
                    ..data.world.read_resource::<MaterialDefaults>().0.clone()
                };

                let (rigid_handle, ground_handle) = {
                    let mut physics_world = data
                        .world
                        .write_resource::<::systems::physics::PhysicsWorld3d>();
                    physics_world.set_gravity(Vector3::new(0.0, 0.0, -9.807));
                    let mut inertia = Inertia::zero();
                    inertia.linear = 1.0;
                    let rigid_handle = physics_world.add_rigid_body(
                        Isometry::new(Vector3::new(-1.5, 0.0, 10.0), Vector3::zeros()),
                        inertia.clone(),
                        Point::origin(),
                    );
                    println!("{:?}", inertia);

                    let ground_handle = BodyHandle::ground();
                    physics_world.add_collider(
                        0.0,
                        ShapeHandle::new(Plane::new(Unit::new_normalize(Vector3::new(
                            0.0, 0.0, 1.0,
                        )))),
                        ground_handle,
                        Isometry::identity(),
                        Material::default(),
                    );

                    physics_world.add_collider(
                        0.0,
                        ShapeHandle::new(Ball::new(5.0)),
                        rigid_handle,
                        Isometry::identity(),
                        Material::default(),
                    );

                    (rigid_handle, ground_handle)
                };

                data.world
                    .create_entity()
                    .with(mesh)
                    .with(material)
                    .with(Transform {
                        translation: ::cgmath::Point3::new(-3., 0., 5.).to_vec(),
                        rotation: ::cgmath::Quaternion::<f32>::one(),
                        scale: ::cgmath::Vector3::from_value(1.),
                    })
                    .with(::systems::physics::Body3d {
                        handle: rigid_handle,
                    })
                    .with(GlobalTransform::default())
                    .build();

                //data.world
                //.create_entity()
                //.with(mesh)
                //.with(material)
                //.with(Transform {
                //translation: ::cgmath::Point3::new(0., 0., 0.).to_vec(),
                //rotation: ::cgmath::Quaternion::<f32>::one(),
                //scale: ::cgmath::Vector3::from_value(1.),
                //})

                let camera_entity = data
                    .world
                    .create_entity()
                    .with(Camera::standard_3d(500., 500.))
                    .with(Transform {
                        translation: ::cgmath::Vector3::new(0., 0., 20.0),
                        rotation: ::cgmath::Quaternion::one(),
                        scale: ::cgmath::Vector3::from_value(1.0),
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

    fn event(&mut self, data: &mut GameData, event: Event) -> Result<Trans<Self>, Error> {
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
                WindowEvent::Destroyed | WindowEvent::CloseRequested => Ok(Trans::Quit),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::S),
                            ..
                        },
                    ..
                } => {
                    let mut physics_world = data
                        .world
                        .write_resource::<::systems::physics::PhysicsWorld3d>();
                    physics_world.step();
                    Ok(Trans::None)
                }
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
