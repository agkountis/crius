use crate::ecs::world::{Universe, World};

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
    fn start(&mut self, _context: Context) {}
    fn stop(&mut self, _context: Context) {}
    fn pause(&mut self, _context: Context) {}
    fn resume(&mut self, _context: Context) {}
    fn handle_event(&mut self, _context: Context) -> Transition {
        Transition::None
    }
    fn update(&mut self, _context: Context) -> Transition {
        Transition::None
    }
    fn pre_draw(&mut self, _context: Context) {}
    fn draw(&mut self, _context: Context) {}
    fn post_draw(&mut self, _context: Context) {}
}

pub struct SceneManager<'a> {
    scenes: Vec<Box<dyn Scene + 'a>>,
    is_running: bool,
}

impl<'a> SceneManager<'a> {
    pub fn new<S>(initial_scene: S) -> Self
    where
        S: Scene + 'a,
    {
        Self {
            scenes: vec![Box::new(initial_scene)],
            is_running: false,
        }
    }

    pub fn initialize(&mut self, context: Context) {
        self.scenes.last_mut().unwrap().start(context);
        self.is_running = true
    }

    pub fn update(&mut self, context: Context) {
        let Context { universe, world } = context;

        if self.is_running {
            let transition = match self.scenes.last_mut() {
                Some(scene) => scene.update(Context::new(universe, world)),
                None => Transition::None,
            };

            self.handle_transition(transition, Context::new(universe, world))
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn handle_transition(&mut self, transition: Transition, context: Context) {
        let Context { universe, world } = context;

        match transition {
            Transition::Push(scene) => self.push(scene, Context::new(universe, world)),
            Transition::Switch(_) => {}
            Transition::Pop => {}
            Transition::None => {}
            Transition::Quit => self.stop(Context::new(universe, world)),
        }
    }

    fn push(&mut self, scene: Box<dyn Scene>, context: Context) {
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
