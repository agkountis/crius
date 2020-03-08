use crius;

use crius::prelude::*;

pub struct MainScene;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Scene for MainScene {
    fn start(&mut self, context: Context<'_>) {
        println!("Starting Scene!");

        let Context { universe: _, world } = context;

        world.insert(
            (),
            (0..10).map(|_| {
                (
                    Position {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Velocity {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                )
            }),
        );
    }
}

#[derive(Debug)]
struct Resource3 {
    pub a: i32,
    pub b: String,
}

fn main() {
    Application::new(MainScene)
        .with_resource(Resource3 {
            a: 30,
            b: "!!!".to_string(),
        })
        .with_system("debug_system", |_, system_builder| {
            system_builder
                .read_resource::<Resource3>()
                .with_query(<(Read<Position>, Read<Velocity>)>::query())
                .build(|_, sub_world, resource3, query| {
                    println!("Debug: {:?} {:?}", resource3.a, resource3.b);
                    query
                        .iter(sub_world)
                        .for_each(|(pos, vel)| println!("Debug: ({:?} {:?})", pos, vel))
                })
        })
        .barrier()
        .with_thread_local_system("thread_local_sys", |_, system_builder| {
            system_builder.build_thread_local(|_, _, _, _| println!("Thread local system"))
        })
        .with_thread_local_fn(|_| println!("Thread local function!"))
        .run()
}
