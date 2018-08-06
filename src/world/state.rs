use amethyst::assets::{Handle, Loader};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::renderer::{
    ActiveCamera, Camera, Event, KeyboardInput, Light, MaterialDefaults, Mesh, PointLight,
    PosNormTangTex, Projection, Rgba, SpotLight, SunLight, VirtualKeyCode, WindowEvent,
};
use amethyst::ui::{Anchor, UiCreator, UiTransform};

use amethyst::prelude::*;

use machinae::{State, Trans};

use world::application::GameData;

use cgmath::{Array, Deg, EuclideanSpace, One};
use nalgebra::core::{Unit, Vector3};
use ncollide3d::shape::{Cuboid, Plane, ShapeHandle};
use nphysics3d::math::Isometry;
use nphysics3d::object::{BodyHandle, BodyMut, Material};
use nphysics3d::volumetric::Volumetric;

use error::Error;

use systems::objects::lights::FlickerLight;
use systems::utils::fps_counter::FPSTag;

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
        info!("{:?} starting", self);
        match *self {
            GameState::Loading => {
                use genmesh::generators::Cube;
                use genmesh::{MapToVertices, Triangulate, Vertices};
                let vertices = Cube::new()
                    .vertex(|v| PosNormTangTex {
                        position: v.pos.into(),
                        normal: v.normal.into(),
                        tangent: [0.1, 0.1, 0.1],
                        tex_coord: [0.1, 0.1],
                    })
                    .triangulate()
                    .vertices()
                    .collect::<Vec<_>>();

                let plane_vertices = ::genmesh::generators::Plane::new()
                    .vertex(|v| PosNormTangTex {
                        position: v.pos.into(),
                        normal: v.normal.into(),
                        tangent: [0.1, 0.1, 0.1],
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

                let plane_mesh: Handle<Mesh> = data.world.read_resource::<Loader>().load_from_data(
                    plane_vertices.into(),
                    (),
                    &data.world.read_resource(),
                );

                let albedo = data.world.read_resource::<Loader>().load_from_data(
                    [1.0, 1.0, 1.0, 1.0].into(),
                    (),
                    &data.world.read_resource(),
                );

                //let plane_albedo = data.world.read_resource::<Loader>().load_from_data(
                //[0.4, 0.4, 0.4, 1.0].into(),
                //(),
                //&data.world.read_resource(),
                //);
                //let plane_material = ::amethyst::renderer::Material {
                //albedo: plane_albedo,
                //..data.world.read_resource::<MaterialDefaults>().0.clone()
                //};

                let (rigid_handle, ground_handle) = {
                    let mut physics_world = data
                        .world
                        .write_resource::<::systems::physics::PhysicsWorld3d>();
                    physics_world.set_gravity(Vector3::z() * -9.807);

                    let cuboid = ShapeHandle::new(Cuboid::new(Vector3::new(1.0, 1.0, 1.0)));
                    let local_inertia = cuboid.inertia(1.0);
                    let local_center_of_mass = cuboid.center_of_mass();
                    let rigid_handle = physics_world.add_rigid_body(
                        Isometry::new(Vector3::new(0.0, 0.0, 50.0), Vector3::zeros()),
                        local_inertia,
                        local_center_of_mass,
                    );

                    {
                        let mut body_mut = physics_world.body_mut(rigid_handle);
                        match body_mut {
                            BodyMut::RigidBody(body) => {
                                body.set_linear_velocity(Vector3::new(0.0, -2.0, -0.2));
                                body.set_angular_velocity(Vector3::new(0.0, 1.8, 3.5));
                            }
                            _ => {}
                        }
                    }

                    let ground_handle = BodyHandle::ground();
                    physics_world.add_collider(
                        0.0,
                        ShapeHandle::new(Plane::new(Unit::new_normalize(Vector3::z()))),
                        ground_handle,
                        Isometry::identity(),
                        Material::default(),
                    );

                    physics_world.add_collider(
                        0.0,
                        cuboid,
                        rigid_handle,
                        Isometry::identity(),
                        Material::default(),
                    );

                    (rigid_handle, ground_handle)
                };

                {
                    let mut fps_entity = None;
                    data.world.exec(|mut creator: UiCreator| {
                        fps_entity = Some(creator.create("resources/ui/fps.ron", ()));
                    });

                    if let Some(fps_entity) = fps_entity {
                        data.world
                            .write_storage::<FPSTag>()
                            .insert(fps_entity, FPSTag);
                    }
                }

                //let ui_transform = UiTransform::new(
                //"fps_text".to_owned(),
                //Anchor::TopLeft,
                //100.0,
                //25.0,
                //1.0,
                //200.0,
                //50.0,
                //0,
                //);
                //let ui_entity = data
                //.world
                //.create_entity()
                //.with(ui_transform)
                ////.with(UiText::new())
                //.with(FPSTag)
                //.build();

                //info!("ui_entity: {:?}", ui_entity);

                let material_defaults = data.world.read_resource::<MaterialDefaults>().0.clone();
                let metallic = [1.0, 1.0, 1.0].into();
                let roughness = [0.6, 0.6, 0.6].into();
                let (metallic, roughness) = {
                    let loader = data.world.read_resource::<Loader>();
                    let textures = &data.world.read_resource();

                    let metallic = loader.load_from_data(metallic, (), textures);
                    let roughness = loader.load_from_data(roughness, (), textures);

                    (metallic, roughness)
                };

                let box_entity = data
                    .world
                    .create_entity()
                    .with(mesh)
                    .with(::amethyst::renderer::Material {
                        albedo: albedo.clone(),
                        metallic: metallic.clone(),
                        roughness: roughness.clone(),
                        ..material_defaults.clone()
                    })
                    .with(Transform {
                        translation: ::cgmath::Point3::new(0.0, 0.0, 0.0).to_vec(),
                        rotation: ::cgmath::Quaternion::<f32>::one(),
                        scale: ::cgmath::Vector3::from_value(1.),
                    })
                    .with(::systems::physics::Body3d {
                        handle: rigid_handle,
                    })
                    .with(GlobalTransform::default())
                    .build();

                let mut plane_transform = Transform::default();
                plane_transform.scale = ::cgmath::Vector3::from_value(1000.0);
                data.world
                    .create_entity()
                    .with(plane_mesh)
                    .with(::amethyst::renderer::Material {
                        albedo: albedo.clone(),
                        metallic: metallic.clone(),
                        roughness: roughness.clone(),
                        ..material_defaults.clone()
                    })
                    .with(plane_transform)
                    .with(::systems::physics::Body3d {
                        handle: ground_handle,
                    })
                    .with(GlobalTransform::default())
                    .build();

                let camera_transform = Transform {
                    translation: ::cgmath::Vector3::new(0., 0.0, 50.0),
                    rotation: ::cgmath::Quaternion::one(),
                    scale: ::cgmath::Vector3::from_value(1.0),
                };
                let camera_entity = data
                    .world
                    .create_entity()
                    .with(::amethyst::controls::FlyControlTag)
                    .with(::systems::controller::FollowCameraTag { entity: box_entity })
                    .with(Camera::from(Projection::perspective(500. / 500., Deg(90.))))
                    .with(camera_transform)
                    .with(GlobalTransform::default())
                    .build();

                let intensity = 4.0;
                let light: Light = PointLight {
                    radius: 1.0,
                    intensity: intensity,
                    color: Rgba(1.0, 0.6, 0.0, 1.0),
                    smoothness: 0.0,
                }.into();

                let mut transform = Transform::default();
                let mut global = GlobalTransform(transform.matrix());
                transform.translation = [0.0, 0.0, 0.1].into();

                let light_entity = data
                    .world
                    .create_entity()
                    .with(light)
                    .with(FlickerLight::new(intensity))
                    .with(transform)
                    .with(global)
                    .build();

                data.world.add_resource(ActiveCamera {
                    entity: camera_entity,
                });

                info!("{:?} switching to {:?}", self, GameState::Menu);
                Ok(Trans::Switch(GameState::Menu))
            }
            GameState::Menu => {
                info!("{:?} switching to {:?}", self, GameState::Running);
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
                _ => Ok(Trans::None),
            },
            _ => Ok(Trans::None),
        }
    }

    fn resume(&mut self, _args: &mut GameData) {
        info!("{:?} resumed", self);
    }

    fn pause(&mut self, _args: &mut GameData) {
        info!("{:?} paused", self);
    }

    fn stop(&mut self, _args: &mut GameData) {
        info!("{:?} stopping", self);
    }
}
