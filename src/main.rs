use bevy::ecs::query;
use::bevy::prelude::*; 
use rand::Rng;
use std::{fmt, default};
use std::collections::HashMap;

struct Player {
    name: String,
    health: i32,
}

struct Equipment {
    weaponName: String,
    baseDamage: i32,
    durability:i32
}
impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".to_string(),
            health: 200,
        }

    }
}
struct EquipmentResources {
    Bullets: u32
}

//Might implement ability for Multiplayer mode later.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum PlayerList {
    Player1,
}

fn main() {

    App::new()
    //Startup systems: Run only once, before anything else, right when the app starts
    .add_system(hello_world)
    .add_system(storyLine)
    .run();
}
fn hello_world() {
    println!("Hello World");
}

fn storyLine() {
    let mut gamer = Player {
       ..Default::default()
    };

    let mut drSmack = Player {
        name: "Dr Smack".to_string(),
        health: 250,
    };

    println!("Welcome to JamiesGame, a gladiator fighting game!");
    println!("Current player status: {}, {}", gamer.name, gamer.health);
    println!("Current enemy status: {}, {}", drSmack.name, drSmack.health);

    
}




