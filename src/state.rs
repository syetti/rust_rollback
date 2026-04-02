// state.rs

// Bitmask for inputs
pub type PlayerInput = u16;

#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerState {
    pub position_x: i32, // Use fixed-point or integers for determinism
    pub position_y: i32,
    pub health: u16,
    pub state_timer: u16,
    pub current_action: u8, // enum mapped to u8 (Idle, Walking, Attacking)
    pub hitstun: u8,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GameState {
    pub frame: u32,
    pub p1: PlayerState,
    pub p2: PlayerState,
}