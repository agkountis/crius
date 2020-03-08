use legion::schedule::{Runnable, Schedulable};
use legion::system::SystemBuilder;
use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;

#[derive(Debug, Clone, Copy)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

pub struct WindowSystem;

impl WindowSystem {
    pub fn new() -> Box<dyn Schedulable> {
        SystemBuilder::new("window_system").build(|_, _, _, _| {
            let event_loop = winit::event_loop::EventLoop::new();

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
        })
    }
}
