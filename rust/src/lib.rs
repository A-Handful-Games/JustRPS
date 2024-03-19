use godot::builtin::meta::registration::method;
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
pub struct Throw {
    sign: matchup::Sign,
    base: Base<Node3D>
}


#[godot_api]
impl INode3D for Throw {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            sign: Sign::Rock,
            base,
        }
    }
}

#[godot_api]
impl Throw {
    #[func]
    fn from_sign(sign: Sign) -> Gd<Self>{
        Gd::from_init_fn(|base| {
            // Accept a base of type Base<Node3D> and directly forward it.
            Self {
                sign: sign.into(),
                base,
            }
        })
    }
    #[func]
    fn start_server() {
        godot_print!("Serving...");
        let _ = networking::serve();
    }
}
