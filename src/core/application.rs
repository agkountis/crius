//! The Application is the core component of crius.
//! It manages the setup of the engine's sub-systems and
//! runs the main loop.
use crate::core::scene::{Context, Scene, SceneManager};
use crate::prelude::Schedule;
use legion::world::{Universe, World};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Application<'a> {
    universe: Universe,
    world: World,
    scene_manager: SceneManager<'a>,
}

impl<'a> Application<'a> {
    pub fn new<S>(initial_scene: S) -> Self
    where
        S: Scene + 'a,
    {
        let universe = Universe::new();
        let world = universe.create_world();
        Self {
            universe,
            world,
            scene_manager: SceneManager::new(initial_scene),
        }
    }

    pub fn run(&mut self, mut schedule: Schedule) {
        self.scene_manager
            .initialize(Context::new(&self.universe, &mut self.world));
        while self.scene_manager.is_running() {
            schedule.execute(&mut self.world)
        }
    }

    pub fn with_resource<R>(&mut self, resource: R) -> &mut Self
    where
        R: Send + Sync + 'static,
    {
        self.world.resources.insert(resource);
        self
    }
}
