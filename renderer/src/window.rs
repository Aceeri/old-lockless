
use {
    winit::{Window, WindowBuilder, EventsLoop},
    rendy::{
        factory::{Config, Factory},
    },
};

pub struct WindowState {
    event_loop: EventsLoop,
    window: Window,
}

impl WindowState {
    pub fn new() -> Result<WindowState, String> {
        let mut event_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title("Lockless")
            .build(&event_loop)?;

        event_loop.poll_events(|_| ());

        Ok(WindowState { event_loop, window })
    }
}

pub struct RenderState {
    factory: rendy::Factory,
}

impl RenderState {
    pub fn new(window: &Window) -> Result<RenderState, String> {
        let config: Config = Default::default();

        let (mut factory, mut families): (Factory<Backend>, _) = rendy::factory::init(config)?;

        let surface = factory.create_surface(&window);

        let mut graph_builder = GraphBuilder::<Backend, Scene<Backend>>::new();

        let size = window
            .get_inner_size()
            .unwrap_or(Default::default())
            .to_physical(window.get_hidpi_factor());

        let window_kind = hal::image::Kind::D2(size.width as u32, size.height as u32, 1, 1);
        let aspect = size.width / size.height;

        let color = graph_builder.create_image(
            window_kind,
            1,
            factory.get_surface_format(&surface),
            Some(hal::command::ClearValue::Color([1.0, 1.0, 1.0, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            hal::format::Format::D16Unorm,
            Some(hal::command::ClearValue::DepthStencil(
                hal::command::ClearDepthStencil(1.0, 0),
            )),
        );

        Ok(RenderState { factory })
    }
}

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
fn window() {
    env_logger::Builder::from_default_env()
        .filter_module("meshes", log::LevelFilter::Trace)
        .init();

    let window_state = WindowState::new().unwrap();
    let render_state = RenderState::new(&window_state.window).unwrap();


    let pass = graph_builder.add_node(
        MeshRenderPipeline::builder()
            .into_subpass()
            .with_color(color)
            .with_depth_stencil(depth)
            .into_pass(),
    );

    let present_builder = PresentNode::builder(&factory, surface, color).with_dependency(pass);

    let frames = present_builder.image_count();

    graph_builder.add_node(present_builder);

    let mut scene = Scene {
        camera: Camera {
            proj: nalgebra::Perspective3::new(aspect as f32, 3.1415 / 4.0, 1.0, 200.0),
            view: nalgebra::Projective3::identity() * nalgebra::Translation3::new(0.0, 0.0, 10.0),
        },
        object_mesh: None,
        objects: vec![],
        lights: vec![
            Light {
                pad: 0.0,
                pos: nalgebra::Vector3::new(0.0, 0.0, 0.0),
                intencity: 10.0,
            },
            Light {
                pad: 0.0,
                pos: nalgebra::Vector3::new(0.0, 20.0, -20.0),
                intencity: 140.0,
            },
            Light {
                pad: 0.0,
                pos: nalgebra::Vector3::new(-20.0, 0.0, -60.0),
                intencity: 100.0,
            },
            Light {
                pad: 0.0,
                pos: nalgebra::Vector3::new(20.0, -30.0, -100.0),
                intencity: 160.0,
            },
        ],
    };

    log::info!("{:#?}", scene);

    let mut graph = graph_builder
        .with_frames_in_flight(frames)
        .build(&mut factory, &mut families, &scene)
        .unwrap();

    let icosphere = genmesh::generators::IcoSphere::subdivide(4);
    let indices: Vec<_> = genmesh::Vertices::vertices(icosphere.indexed_polygon_iter())
        .map(|i| i as u32)
        .collect();
    let vertices: Vec<_> = icosphere
        .shared_vertex_iter()
        .map(|v| PosColorNorm {
            position: v.pos.into(),
            color: [
                (v.pos.x + 1.0) / 2.0,
                (v.pos.y + 1.0) / 2.0,
                (v.pos.z + 1.0) / 2.0,
                1.0,
            ]
            .into(),
            normal: v.normal.into(),
        })
        .collect();

    scene.object_mesh = Some(
        Mesh::<Backend>::builder()
            .with_indices(&indices[..])
            .with_vertices(&vertices[..])
            .build(graph.node_queue(pass), &factory)
            .unwrap(),
    );

    let started = time::Instant::now();

    let mut frames = 0u64..;
    let mut rng = rand::thread_rng();
    let rxy = Uniform::new(-1.0, 1.0);
    let rz = Uniform::new(0.0, 185.0);

    let mut fpss = Vec::new();
    let mut checkpoint = started;
    let mut should_close = false;

    while !should_close && scene.objects.len() < MAX_OBJECTS {
        let start = frames.start;
        let from = scene.objects.len();
        for _ in &mut frames {
            factory.maintain(&mut families);
            event_loop.poll_events(|event| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => should_close = true,
                _ => (),
            });
            graph.run(&mut factory, &mut families, &scene);

            let elapsed = checkpoint.elapsed();

            if scene.objects.len() < MAX_OBJECTS {
                scene.objects.push({
                    let z = rz.sample(&mut rng);
                    nalgebra::Transform3::identity()
                        * nalgebra::Translation3::new(
                            rxy.sample(&mut rng) * (z / 2.0 + 4.0),
                            rxy.sample(&mut rng) * (z / 2.0 + 4.0),
                            -z,
                        )
                })
            }

            if should_close
                || elapsed > std::time::Duration::new(5, 0)
                || scene.objects.len() == MAX_OBJECTS
            {
                let frames = frames.start - start;
                let nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
                fpss.push((
                    frames * 1_000_000_000 / nanos,
                    from..scene.objects.len(),
                ));
                checkpoint += elapsed;
                break;
            }
        }
    }

    log::info!("FPS: {:#?}", fpss);

    graph.dispose(&mut factory, &scene);
}
