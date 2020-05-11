use crius::event::{EventChannel, KeyboardInput, VirtualKeyCode, WindowEvent};
use crius::graphics::{
    api::image::{Image, ImageView},
    Graphics,
};
use crius::prelude::*;
use std::sync::Arc;
use winit::event::ElementState;

pub struct MainScene;

#[derive(Debug, Clone)]
pub enum MyCustomEvent {
    Foo,
    Bla,
}

impl Scene for MainScene {
    fn start(&mut self, context: Context) {
        let Context { world, .. } = context;

        if let Some(graphics) = world.resources.get::<Arc<Graphics>>() {
            let image = Image::new(
                Arc::clone(&graphics),
                ash::vk::Extent3D {
                    width: 256,
                    height: 256,
                    depth: 1,
                },
                ash::vk::Format::R8G8B8A8_SRGB,
                ash::vk::ImageUsageFlags::SAMPLED,
                vk_mem::MemoryUsage::GpuOnly,
                ash::vk::SampleCountFlags::TYPE_1,
                ash::vk::ImageTiling::OPTIMAL,
                1,
                1,
                ash::vk::ImageCreateFlags::empty(),
            )
            .unwrap();

            let image_view = ImageView::new(
                Arc::clone(&graphics),
                &image,
                image.format(),
                ash::vk::ImageViewType::TYPE_2D,
                ash::vk::ImageAspectFlags::COLOR,
                ash::vk::ComponentMapping {
                    r: ash::vk::ComponentSwizzle::R,
                    g: ash::vk::ComponentSwizzle::G,
                    b: ash::vk::ComponentSwizzle::B,
                    a: ash::vk::ComponentSwizzle::A,
                },
                0,
                1,
                0,
                1,
            )
            .unwrap();

            println!("Starting!")
        } else {
            println!("Graphics not found!")
        }
    }

    fn stop(&mut self, context: Context) {
        println!("Stopping scene");
    }

    fn handle_event(&mut self, context: Context, event: Event) -> Transition {
        let Context { world, .. } = context;
        match event {
            Event::Application(event) => println!("EVENT: {:?}", event),
            Event::Window(event) => {
                println!("EVENT: {:?}", event);
                match event {
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key),
                                modifiers,
                                ..
                            },
                        ..
                    } => match key {
                        VirtualKeyCode::A if modifiers.shift() => {
                            if let Some(channel) =
                                world.resources.get_mut::<EventChannel<MyCustomEvent>>()
                            {
                                channel.write(MyCustomEvent::Foo).unwrap()
                            }
                        }
                        VirtualKeyCode::S => {
                            if let Some(channel) =
                                world.resources.get_mut::<EventChannel<MyCustomEvent>>()
                            {
                                channel.write(MyCustomEvent::Bla).unwrap()
                            }
                        }
                        VirtualKeyCode::Escape => return Transition::Quit,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Transition::None
    }
}

fn main() {
    let mut channel = EventChannel::<MyCustomEvent>::default();
    let reader = channel.bind_listener(128);
    ApplicationBuilder::new(MainScene, "examples/playground")
        .with_resource(channel)
        .with_system("debug_system", move |_, system_builder| {
            system_builder
                .read_resource::<EventChannel<MyCustomEvent>>()
                .build(move |_, _, user_event_channel, _| {
                    while let Some(e) = user_event_channel.read(reader) {
                        println!("debug_system received user event: {:?}", e)
                    }
                })
        })
        .build()
        .run()
}
