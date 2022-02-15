use bevy::ecs::query;
use::bevy::prelude::*; 

struct Player {
    name: String,
    health: i32,
    damage: i32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".to_string(),
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
    commands.spawn().insert(Player).insert(health("50".parse::<i32>().unwrap()));
    println()
}
    
