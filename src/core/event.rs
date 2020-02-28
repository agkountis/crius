use std::path::PathBuf;
use winit::event::Event;

pub enum SceneEvent<'a, T: 'static> {
    Window(Event<'a, T>)
}
