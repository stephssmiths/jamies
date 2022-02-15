use bevy::ecs::query;
use::bevy::prelude::*; 

struct Player {
    name: String,
    health: f32,
    damage: f32,
}

impl BaseStats for Player {
    fn basestats() -> Self {
        Player {
            name: "Player",
            health: 100,
            damage: 10,
        }

    }
}

struct Equipment {
    fist: f32,
    stick: f32,
    glock: f32,
    baseGrenade: f32,
    molotov: f32
}

fn main() {

    App::new()
    //Startup systems: Run only once, before anything else, right when the app starts
    .add_system(hello_world)
    .run();
}
fn hello_world() {
    println!("Hello World");
}

fn add_people(mut commands: Commands) {
    //Using Commands to spawn entities into the world.
    commands.spawn().insert(Person).insert(Health("132".parse::<i32>().unwrap()));

}
    
}