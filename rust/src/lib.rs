use godot::prelude::*;
use godot::engine::{Sprite2D, ISprite2D};
use crate::rps::matchup::{self, *};

pub mod rps;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Throw {
    sign: matchup::Sign,
    base: Base<Sprite2D>
}


#[godot_api]
impl ISprite2D for Throw {
    fn init(base: Base<Sprite2D>) -> Self {
        let s = format!("{:?}", fight(&Sign::Rock, &Sign::Scissors));
        godot_print!("Rock attacking Scissors is a: {s}"); // Prints to the Godot console
        
        Self {
            sign: Sign::Rock,
            base,
        }
    }
}