use legion;
use legion::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(f32, f32, f32);
#[derive(Clone, Copy, Debug, PartialEq)]
struct Vel(f32, f32, f32);

#[derive(Clone)]
pub struct ExampleResource1(String);
#[derive(Clone)]
pub struct ExampleResource2(String);

fn main() {
    // create world
    let universe = Universe::new();
    let mut world = universe.create_world();

    world
        .resources
        .insert(ExampleResource1("ExampleResource1".to_string()));
    world
        .resources
        .insert(ExampleResource2("ExampleResource2".to_string()));

    // create entities
    // An insert call is used to insert matching entities into the world.
    world.insert(
        (),
        vec![
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
            (Pos(1., 2., 3.), Vel(1., 2., 3.)),
        ],
    );

    // update positions using a system
    let update_positions = SystemBuilder::new("update_positions")
        .write_resource::<ExampleResource1>()
        .read_resource::<ExampleResource2>()
        .with_query(<(Write<Pos>, Read<Vel>)>::query())
        .build(|_, mut world, (res1, res2), query| {
            res1.0 = res2.0.clone(); // Write the mutable resource from the immutable resource

            query.iter(&mut world).for_each(|(mut pos, vel)| {
                pos.0 += vel.0;
                pos.1 += vel.1;
                pos.2 += vel.2;
            });
        });

    let print_system = SystemBuilder::new("print_system")
        .with_query(<Read<Pos>>::query())
        .build(|_, mut world, _, query| {
            query
                .iter(&mut world)
                .for_each(|pos| println!("Position {} {} {}", pos.0, pos.1, pos.2))
        });

    let print_system2 = SystemBuilder::new("printer2")
        .build(|_, _, _, _| (0..10000).for_each(|a| println!("printer2 {}", a)));

    let print_system3 = SystemBuilder::new("printer3")
        .build(|_, _, _, _| (0..10000).for_each(|a| println!("printer3 {}", a)));

    let mut schedule = Schedule::builder()
        .add_system(print_system)
        .add_system(print_system2)
        .add_system(print_system3)
        .add_system(update_positions)
        .build();

    // Execute a frame of the schedule.
    schedule.execute(&mut world);
}
