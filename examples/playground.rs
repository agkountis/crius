use crius;

use crius::prelude::*;
use crius::window::WindowSystem;
use legion::schedule::{Runnable, Schedulable};

pub struct MainScene;

impl Scene for MainScene {
    fn start(&mut self, _context: Context<'_>) {
        println!("Starting Scene!")
    }
}

struct Resource3 {
    a: i32,
    b: String,
}

fn main() {
    let resource1 = (10, "Hello");
    let resource2 = (20, "World");

    Application::new(MainScene)
        .with_resource(resource1)
        .with_resource(resource2)
        .with_resource(Resource3 {
            a: 30,
            b: "!!!".to_string(),
        })
        .run(Schedule::builder().add_system(WindowSystem::new()).build())
}
