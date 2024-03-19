use godot::prelude::*;
use godot::engine::{Node3D, INode3D};
use crate::rps::matchup::{self, *};

pub mod rps;
pub mod networking;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Throw {
    sign: matchup::Sign,
    base: Base<Node3D>
}


#[godot_api]
impl INode3D for Throw {
    fn init(base: Base<Node3D>) -> Self {
        let s = format!("{:?}", fight(&Sign::Rock, &Sign::Scissors));
        godot_print!("Rock attacking Scissors is a: {s}"); // Prints to the Godot console
        godot_print!("Serving...");
        let _ = networking::serve();
        Self {
            sign: Sign::Rock,
            base,
        }
    }
}