pub type WindowEvent<'a> = winit::event::WindowEvent<'a>;
pub type VirtualKeyCode = winit::event::VirtualKeyCode;
pub type KeyboardInput = winit::event::KeyboardInput;

pub type EventChannel<T> = legion::event::Channel<T>;

#[derive(Debug)]
pub enum ApplicationEvent {
    Suspended,
    Resumed,
    Terminating,
}

#[derive(Debug)]
pub enum Event<'a> {
    Application(ApplicationEvent),
    Window(WindowEvent<'a>),
}
