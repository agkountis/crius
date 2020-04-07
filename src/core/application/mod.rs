//! The Application is the core component of crius.
//! It manages the setup of the engine's sub-systems and
//! runs the main loop.

pub mod settings;

use crate::core::application::settings::Settings;
use crate::core::event::ApplicationEvent;
use crate::core::scene::{Context, Scene, SceneManager, Transition};
use crate::prelude::{Event, Schedule};
use legion::schedule::{Builder, Runnable, Schedulable};
use legion::system::SystemBuilder;
use legion::world::{Universe, World};
use serde_yaml;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use winit::event::Event as WinitEvent;
use winit::event::WindowEvent as WinitWindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowBuilder};

const APPLICATION_SETTINGS_FILE_NAME: &str = "settings.yml";

pub struct Application {
    universe: Universe,
    world: World,
    scene_manager: SceneManager,
    schedule: Schedule,
    settings: Settings,
}

impl Application {
    pub fn run(self) {
        let Application {
            mut universe,
            mut world,
            mut scene_manager,
            mut schedule,
            settings,
        } = self;

        scene_manager.initialize(Context::new(&mut universe, &mut world));

        let event_loop = winit::event_loop::EventLoop::new();

        let mut window_builder = WindowBuilder::new()
            .with_title(settings.window.title)
            .with_resizable(settings.window.resizeable)
            .with_maximized(settings.window.maximized)
            .with_visible(settings.window.visible)
            .with_transparent(settings.window.transparent)
            .with_decorations(settings.window.decorations)
            .with_always_on_top(settings.window.always_on_top);

        if let Some(size) = settings.window.size {
            window_builder = window_builder.with_inner_size(size);
        }

        if let Some(min) = settings.window.min_size {
            window_builder = window_builder.with_max_inner_size(min);
        }

        if let Some(max) = settings.window.max_size {
            window_builder = window_builder.with_max_inner_size(max);
        }

        let window = window_builder.build(&event_loop).unwrap();

        world.resources.insert(window);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                WinitEvent::WindowEvent {
                    event: WinitWindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                WinitEvent::WindowEvent { event, .. } => {
                    let transition = scene_manager
                        .handle_event(Context::new(&universe, &mut world), Event::Window(event));
                    Self::handle_transition(
                        &mut scene_manager,
                        transition,
                        Context::new(&universe, &mut world),
                        control_flow,
                    )
                }
                WinitEvent::Suspended => {
                    let transition = scene_manager.handle_event(
                        Context::new(&universe, &mut world),
                        Event::Application(ApplicationEvent::Suspended),
                    );
                    scene_manager.pause(Context::new(&mut universe, &mut world));
                    *control_flow = ControlFlow::Wait;
                    Self::handle_transition(
                        &mut scene_manager,
                        transition,
                        Context::new(&universe, &mut world),
                        control_flow,
                    );
                }
                WinitEvent::Resumed => {
                    let transition = scene_manager.handle_event(
                        Context::new(&universe, &mut world),
                        Event::Application(ApplicationEvent::Resumed),
                    );
                    scene_manager.resume(Context::new(&universe, &mut world));
                    *control_flow = ControlFlow::Poll;
                    Self::handle_transition(
                        &mut scene_manager,
                        transition,
                        Context::new(&universe, &mut world),
                        control_flow,
                    )
                }
                WinitEvent::MainEventsCleared => {
                    let transition = scene_manager.update(Context::new(&mut universe, &mut world));
                    Self::handle_transition(
                        &mut scene_manager,
                        transition,
                        Context::new(&universe, &mut world),
                        control_flow,
                    );
                    world.resources.get::<Window>().unwrap().request_redraw()
                }
                WinitEvent::RedrawRequested(_) => {
                    schedule.execute(&mut world);
                }
                WinitEvent::LoopDestroyed => {
                    // Event loop is being destroyed, no more transitions will be handled.
                    scene_manager.handle_event(
                        Context::new(&universe, &mut world),
                        Event::Application(ApplicationEvent::Terminating),
                    );
                    scene_manager.stop(Context::new(&mut universe, &mut world))
                }
                _ => {}
            }
        })
    }

    fn handle_transition(
        scene_manager: &mut SceneManager,
        transition: Transition,
        context: Context,
        control_flow: &mut ControlFlow,
    ) {
        let Context { universe, world } = context;

        match transition {
            Transition::Push(scene) => scene_manager.push(scene, Context::new(universe, world)),
            Transition::Switch(_) => {}
            Transition::Pop => {}
            Transition::Quit => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    }
}

pub struct ApplicationBuilder<P>
where
    P: AsRef<Path>,
{
    universe: Universe,
    world: World,
    scene_manager: SceneManager,
    schedule_builder: Builder,
    working_directory: P,
}

impl<P> ApplicationBuilder<P>
where
    P: AsRef<Path>,
{
    pub fn new<S>(initial_scene: S, working_directory: P) -> Self
    where
        S: Scene + 'static,
    {
        let universe = Universe::new();
        let world = universe.create_world();
        Self {
            universe,
            world,
            scene_manager: SceneManager::new(initial_scene),
            schedule_builder: Schedule::builder(),
            working_directory,
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

    pub fn flush(mut self) -> Self {
        self.schedule_builder = self.schedule_builder.flush();
        self
    }

    pub fn build(self) -> Application {
        let settings: Settings = {
            let mut file = File::open(
                self.working_directory
                    .as_ref()
                    .join(APPLICATION_SETTINGS_FILE_NAME),
            )
            .expect("Failed to open settings.yml file.");
            let mut buffer = vec![];
            file.read_to_end(&mut buffer)
                .expect("Failed to read settings.yml file");

            serde_yaml::from_slice(buffer.as_slice()).expect("Failed to deserialize settings.yml.")
        };

        Application {
            universe: self.universe,
            world: self.world,
            scene_manager: self.scene_manager,
            schedule: self.schedule_builder.build(),
            settings,
        }
    }
}
