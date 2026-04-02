// lib.rs
// 1. Declare the modules so Rust knows to compile those files
mod state;
mod rollback_core;

// 2. Bring the required structs/constants into scope
use rollback_core::RollbackCore;
use crate::rollback_core::MAX_ROLLBACK_FRAMES;

use godot::prelude::*;

struct MyRollbackExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyRollbackExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FgSession {
    core: RollbackCore,
    base: Base<Node>,
}

#[godot_api]
impl INode for FgSession {
    fn init(base: Base<Node>) -> Self {
        Self {
            core: RollbackCore::new(),
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        // In Godot, you would read local controller inputs here
        // send them to the network, and push them to the core.
        
        self.core.advance_frame();
    }
}

#[godot_api]
impl FgSession {
    // Expose methods for GDScript to read the state and update visuals
    #[func]
    pub fn get_p1_x(&self) -> i32 {
        let current_state = self.core.state_history[(self.core.current_frame as usize) % MAX_ROLLBACK_FRAMES];
        current_state.p1.position_x
    }

    #[func]
    pub fn receive_network_packet(&mut self, frame: u32, input: i32) {
        self.core.process_network_input(frame, input as u16);
    }
}