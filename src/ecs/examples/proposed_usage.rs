//! Proposed API usage for when the ECS library is complete.

extern crate amethyst_ecs;
use amethyst_ecs as ecs;

// Define our processors.

struct Rendering;
impl ecs::Processor for Rendering {
    fn setup(&mut self) {}

    fn process(&mut self, world: &mut ecs::World) {
        println!("position {:?}", world.component::<Position>(0).unwrap());
        println!("Tick!");
    }
}

struct Physics {
    vel: f32
}

impl ecs::Processor for Physics {
    fn setup(&mut self) {}

    fn process(&mut self, world: &mut ecs::World) {
        let mut position = &mut world.component_mut::<Position>(0).unwrap().1;
        position.x += self.vel;
    }
}

// Define our components.

#[allow(dead_code)]
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let mut world = ecs::World::new();

    let sim_result = ecs::Simulation::build()
                         .with(Physics { vel: 1.0 })
                         .with(Rendering)
                         .done();

    let mut sim = match sim_result {
        Err(e) => panic!("Simulation couldn't be built due to: {:?}", e),
        Ok(sim) => sim,
    };

    let mut world = world.build_entity()
                   .with(Position { x: 0.0, y: 0.0, z: 0.0, })
                   .done()
                   .unwrap();

    for _ in 0..5 {
        // Put game logic here.

        // TODO: Add `Duration` param to `step()` method. Not added because of
        // possible circular dep with `amethyst_engine`, which re-exports the
        // time crate's `Duration` type.
        //
        // let dt = get_delta_time_from_somewhere();
        // world = sim.step(world, dt);
        world = sim.step(world);
    }
}
