//! The Application is the core component of crius.
//! It manages the setup of the engine's sub-systems and
//! runs the main loop.
use crate::core::scene::{Context, Scene, SceneManager};
use crate::prelude::Schedule;
use legion::schedule::{Builder, Runnable, Schedulable};
use legion::system::SystemBuilder;
use legion::world::{Universe, World};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Application<'a> {
    universe: Universe,
    world: World,
    scene_manager: SceneManager<'a>,
    schedule_builder: Builder,
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
            schedule_builder: Schedule::builder(),
        }
    }

    pub fn run(mut self) {
        self.scene_manager
            .initialize(Context::new(&self.universe, &mut self.world));

        let mut schedule = self.schedule_builder.build();
        while self.scene_manager.is_running() {
            self.scene_manager
                .update(Context::new(&self.universe, &mut self.world));
            schedule.execute(&mut self.world)
        }
    }

    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: Send + Sync + 'static,
    {
        self.world.resources.insert(resource);
        self
    }

    pub fn with_system<B>(mut self, name: &'static str, mut builder_func: B) -> Self
    where
        B: FnMut(&mut World, SystemBuilder) -> Box<dyn Schedulable>,
    {
        self.schedule_builder = self
            .schedule_builder
            .add_system(builder_func(&mut self.world, SystemBuilder::new(name)));
        self
    }

    pub fn with_thread_local_system<B>(mut self, name: &'static str, mut builder_func: B) -> Self
    where
        B: FnMut(&mut World, SystemBuilder) -> Box<dyn Runnable>,
    {
        self.schedule_builder = self
            .schedule_builder
            .add_thread_local(builder_func(&mut self.world, SystemBuilder::new(name)));
        self
    }

    pub fn with_thread_local_fn<F>(mut self, func: F) -> Self
    where
        F: FnMut(&mut World) + 'static,
    {
        self.schedule_builder = self.schedule_builder.add_thread_local_fn(func);
        self
    }

    pub fn barrier(mut self) -> Self {
        self.schedule_builder = self.schedule_builder.flush();
        self
    }
}
