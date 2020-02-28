use crate::core::event::SceneEvent;
use legion::world::{Universe, World};

pub enum Transition<T> {
    Push(Box<dyn Scene<T>>),
    Switch(Box<dyn Scene<T>>),
    Pop,
    None,
    Quit,
}

pub struct Context<'a, T> {
    universe: &'a Universe,
    world: &'a mut World,
    data: &'a T,
}

impl<'a, T> Context<'a, T>
where
    T: 'a,
{
    pub fn new(universe: &'a Universe, world: &'a mut World, data: &'a T) -> Self {
        Self {
            universe,
            world,
            data,
        }
    }
}

//TODO: Add fixed update and late update
pub trait Scene<T> {
    fn start(&mut self, _context: Context<T>) {}
    fn stop(&mut self, _context: Context<T>) {}
    fn pause(&mut self, _context: Context<T>) {}
    fn resume(&mut self, _context: Context<T>) {}
    fn handle_event(&mut self, _context: Context<T>) -> Transition<T> {
        Transition::None
    }
    fn update(&mut self, _context: Context<T>) -> Transition<T> {
        Transition::None
    }
    fn pre_draw(&mut self, _context: Context<T>) {}
    fn draw(&mut self, _context: Context<T>) {}
    fn post_draw(&mut self, _context: Context<T>) {}
}

pub struct SceneManager<'a, T> {
    scenes: Vec<Box<dyn Scene<T> + 'a>>,
    is_running: bool,
}

impl<'a, T> SceneManager<'a, T> {
    pub fn new<S>(initial_scene: S) -> Self
    where
        S: Scene<T> + 'a,
    {
        Self {
            scenes: vec![Box::new(initial_scene)],
            is_running: false,
        }
    }

    pub fn initialize(&mut self, context: Context<T>) {
        self.scenes.last_mut().unwrap().start(context);
        self.is_running = true
    }

    pub fn handle_event(&mut self, context: Context<T>, event: SceneEvent<'a, ()>) {
        let Context {
            universe,
            world,
            data,
        } = context;

        if self.is_running {
            let transition = match self.scenes.last_mut() {
                Some(scene) => scene.update(Context::new(universe, world, data)),
                None => Transition::None,
            };

            self.handle_transition(transition, Context::new(universe, world, data))
        }
    }

    fn handle_transition(&mut self, transition: Transition<T>, context: Context<T>) {
        let Context {
            universe,
            world,
            data,
        } = context;

        match transition {
            Transition::Push(scene) => self.push(scene, Context::new(universe, world, data)),
            Transition::Switch(_) => {}
            Transition::Pop => {}
            Transition::None => {}
            Transition::Quit => self.stop(Context::new(universe, world, data)),
        }
    }

    fn push(&mut self, scene: Box<dyn Scene<T>>, context: Context<T>) {
        let Context {
            universe,
            world,
            data,
        } = context;

        if let Some(current) = self.scenes.last_mut() {
            current.pause(Context::new(universe, world, data))
        }

        self.scenes.push(scene);
        self.scenes
            .last_mut()
            .unwrap()
            .start(Context::new(universe, world, data))
    }

    pub(crate) fn stop(&mut self, context: Context<T>) {
        if self.is_running {
            let Context {
                universe,
                world,
                data,
            } = context;

            while let Some(mut scene) = self.scenes.pop() {
                scene.stop(Context::new(universe, world, data))
            }

            self.is_running = false;
        }
    }
}
