// rollback_core.rs

use crate::state::GameState;
use crate::state::PlayerInput;
pub const MAX_ROLLBACK_FRAMES: usize = 60;


pub struct RollbackCore {
    // Circular buffer for game states and inputs
    pub state_history: [GameState; MAX_ROLLBACK_FRAMES],
    pub p1_inputs: [PlayerInput; MAX_ROLLBACK_FRAMES],
    pub p2_inputs: [PlayerInput; MAX_ROLLBACK_FRAMES],

    pub current_frame: u32,
    pub confirmed_frame: u32, // The last frame where both players' inputs are known and confirmed
}

impl RollbackCore {
    pub fn new() -> Self {
        Self {
            state_history: [GameState::default(); MAX_ROLLBACK_FRAMES],
            p1_inputs: [0; MAX_ROLLBACK_FRAMES],
            p2_inputs: [0; MAX_ROLLBACK_FRAMES],
            current_frame: 0,
            confirmed_frame: 0,
        }
    }

    pub fn advance_frame(&mut self) {
        // 1. Get the current state
        let mut current_state = self.state_history[(self.current_frame as usize) % MAX_ROLLBACK_FRAMES];
        
        // 2. Fetch inputs
        let p1_input = self.p1_inputs[(self.current_frame as usize) % MAX_ROLLBACK_FRAMES];
        let p2_input = self.p2_inputs[(self.current_frame as usize) % MAX_ROLLBACK_FRAMES];

        // 3. APPLY FIGHTING GAME LOGIC HERE
        // For now, let's map input '1' to action '1' (Punch)
        if p1_input == 1 {
            current_state.p1.current_action = 1;
        } else {
            current_state.p1.current_action = 0; // Idle
        }

        if p2_input == 1 {
            current_state.p2.current_action = 1;
        } else {
            current_state.p2.current_action = 0; // Idle
        }
        
        current_state.frame += 1;
        self.current_frame += 1;
        
        // 4. Save new state
        self.state_history[(self.current_frame as usize) % MAX_ROLLBACK_FRAMES] = current_state;
    }

    pub fn process_network_input(&mut self, remote_frame: u32, remote_input: PlayerInput) {
        if remote_frame <= self.confirmed_frame { return; } // Already processed

        // Insert input into buffer
        self.p2_inputs[(remote_frame as usize) % MAX_ROLLBACK_FRAMES] = remote_input;

        // If the input was for a past frame, trigger rollback
        if remote_frame < self.current_frame {
            self.rollback_to(remote_frame);
        }
    }

    fn rollback_to(&mut self, frame: u32) {
        let frames_to_resimulate = self.current_frame - frame;
        self.current_frame = frame;
        
        // Rapidly resimulate back to the present
        for _ in 0..frames_to_resimulate {
            self.advance_frame();
        }
    }
}

//Tests
#[cfg(test)]
mod tests {
    use super::*; // Brings RollbackCore into scope
    use crate::state::PlayerInput;

    #[test]
    fn test_basic_frame_advancement() {
        let mut core = RollbackCore::new();
        assert_eq!(core.current_frame, 0);
        
        core.advance_frame();
        assert_eq!(core.current_frame, 1);
    }

    #[test]
    fn test_rollback_consistency() {
        let mut core = RollbackCore::new();
        
        // define an input (e.g., 1 =  Punch)
        let punch_input: PlayerInput = 1; 

        // 1. Advance to frame 5 normally
        for _ in 0..5 {
            core.advance_frame();
        }
        
        // 2. Save a snapshot of the state at frame 5 (Timeline A)
        let state_without_punch = core.state_history[5 % MAX_ROLLBACK_FRAMES];

        // 3. Suddenly, a delayed network packet arrives! 
        // Player 2 actually pressed "Punch" on frame 2.
        core.process_network_input(2, punch_input);

        // process_network_input automatically rolled us back to frame 2 
        // and fast-forwarded us back to frame 5.
        
        // 4. Verify Timeline B
        let state_with_punch = core.state_history[5 % MAX_ROLLBACK_FRAMES];
        assert_eq!(core.current_frame, 5, "We should be back at frame 5");
        
        // If your logic is correct, Player 2's state should now be different 
        // than it was in Timeline A.
        assert_ne!(
            state_without_punch.p2.current_action, 
            state_with_punch.p2.current_action,
            "Rollback failed to apply the delayed punch input!"
        );
    }
}