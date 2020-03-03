use crius;

use crius::prelude::*;

pub struct MainScene;

impl Scene<()> for MainScene {}

fn main() {
    let resource1 = (10, "Hello");
    let resource2 = (20, "World");

    Application::new(MainScene)
        .with_resource(resource1)
        .with_resource(resource2)
        .run()
}
