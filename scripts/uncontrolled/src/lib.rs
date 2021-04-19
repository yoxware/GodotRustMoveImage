#[macro_use]
extern crate gdnative;

use gdnative::api::MeshInstance;
use gdnative::prelude::NativeClass;
use gdnative::prelude::Sprite;
use gdnative::prelude::*;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Sprite)]
struct Uncontrolled {}

#[gdnative::methods]
impl Uncontrolled {
    fn _init(_owner: gdnative::Sprite) -> Self {
        Uncontrolled {}
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: gdnative::Sprite) {
        godot_warn!("Start");
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Sprite) {
        godot_dbg!("process called");
        let mut pos = owner.get_position();
        pos.x += 1.0;
        owner.set_position(pos);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Uncontrolled>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
