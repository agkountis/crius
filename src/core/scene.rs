use crate::ecs::world::{Universe, World};
use crate::event::Event;

pub enum Transition {
    Push(Box<dyn Scene>),
    Switch(Box<dyn Scene>),
    Pop,
    None,
    Quit,
}

pub struct Context<'a> {
    pub universe: &'a Universe,
    pub world: &'a mut World,
}

impl<'a> Context<'a> {
    pub fn new(universe: &'a Universe, world: &'a mut World) -> Self {
        Self { universe, world }
    }
}

//TODO: Add fixed update and late update
pub trait Scene {
    fn start(&mut self, context: Context) {}
    fn stop(&mut self, context: Context) {}
    fn pause(&mut self, context: Context) {}
    fn resume(&mut self, context: Context) {}
    fn handle_event(&mut self, context: Context, event: Event) -> Transition {
        Transition::None
    }
    fn update(&mut self, context: Context) -> Transition {
        Transition::None
    }
    fn pre_draw(&mut self, context: Context) {}
    fn draw(&mut self, context: Context) {}
    fn post_draw(&mut self, context: Context) {}
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    is_running: bool,
}

impl SceneManager {
    pub fn new<S>(initial_scene: S) -> Self
    where
        S: Scene + 'static,
    {
        Self {
            scenes: vec![Box::new(initial_scene)],
            is_running: false,
        }
    }

    pub(crate) fn initialize(&mut self, context: Context) {
        self.scenes.last_mut().unwrap().start(context);
        self.is_running = true
    }

    pub(crate) fn update(&mut self, context: Context) -> Transition {
        let Context { universe, world } = context;

        match self.scenes.last_mut() {
            Some(scene) => scene.update(Context::new(universe, world)),
            None => Transition::None,
        }
    }

    pub(crate) fn handle_event(&mut self, context: Context, event: Event) -> Transition {
        let Context { universe, world } = context;

        match self.scenes.last_mut() {
            Some(scene) => scene.handle_event(Context::new(universe, world), event),
            None => Transition::None,
        }
    }

    pub(crate) fn pause(&mut self, context: Context) {
        let Context { universe, world } = context;

        if let Some(scene) = self.scenes.last_mut() {
            scene.pause(Context::new(universe, world))
        }
    }

    pub(crate) fn resume(&mut self, context: Context) {
        let Context { universe, world } = context;

        if let Some(scene) = self.scenes.last_mut() {
            scene.resume(Context::new(universe, world))
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub(crate) fn push(&mut self, scene: Box<dyn Scene>, context: Context) {
        let Context { universe, world } = context;

        if let Some(current) = self.scenes.last_mut() {
            current.pause(Context::new(universe, world))
        }

        self.scenes.push(scene);
        self.scenes
            .last_mut()
            .unwrap()
            .start(Context::new(universe, world))
    }

    pub(crate) fn stop(&mut self, context: Context) {
        if self.is_running {
            let Context { universe, world } = context;

            while let Some(mut scene) = self.scenes.pop() {
                scene.stop(Context::new(universe, world))
            }

            self.is_running = false;
        }
    }
}
