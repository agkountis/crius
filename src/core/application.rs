//! The Application is the core component of crius.
//! It manages the setup of the engine's sub-systems and
//! runs the main loop.
use crate::core::scene::{Scene, SceneManager};
use legion::resource::Resources;
use legion::world::{Universe, World};
use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowBuilder};

pub struct Application<'a, T>
where
    T: 'static,
{
    universe: Universe,
    world: World,
    scene_manager: SceneManager<'a, T>,
}

impl<'a, T> Application<'a, T> {
    pub fn new<S: Scene<T> + 'a>(initial_scene: S) -> Self {
        let universe = Universe::new();
        let world = universe.create_world();
        Self {
            universe,
            world,
            scene_manager: SceneManager::new(initial_scene),
        }
    }

    pub fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new();

        // TODO: Move event loop and window management into separate ECS systems.
        let window = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { .. } => println!("Device event"),
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::MainEventsCleared => {}
                Event::RedrawRequested(_) => {}
                Event::RedrawEventsCleared => {}
                Event::LoopDestroyed => {}
            }
        })
    }

    pub fn with_resource<R>(&mut self, resource: R) -> &mut Self
    where
        R: Send + Sync + 'static,
    {
        self.world.resources.insert(resource);
        self
    }
}
