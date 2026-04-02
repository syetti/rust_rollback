# Migration Blueprint: Pure Rust Rollback

## Phase 1: Pure Data State (No Godot Types)
**Goal:** Create a single struct that holds the entire game state.
- [ ] Define inputs as a bitmask (`u16`).
- [ ] Define character states as strict enums (`PlayerAction`).
- [ ] Replace `Vector2` / `float` with `i32` or `fixed::types::I16F16`.
- [ ] Ensure `GameState` strictly derives `Clone, Copy, Default`.

## Phase 2: The Logic Loop ( advance_frame )
**Goal:** Rust simulates the game math without Godot physics.
- [ ] Map raw input bitmasks to velocity changes (e.g., `if input & INPUT_RIGHT != 0 { velocity_x = 100 }`).
- [ ] Implement `state_timer` to lock characters into attack/hitstun animations.
- [ ] Write a custom AABB collision function to check if Player 1's attack box overlaps Player 2's hurtbox.

## Phase 3: The Godot Interface (lib.rs)
**Goal:** Expose the Rust data so Godot can draw it.
- [ ] Expose `get_p1_x()`, `get_p1_y()`, and `get_p1_action()` to GDScript via `#[func]`.
- [ ] Create a `advance_simulation(p1_input: int)` function that Godot calls inside its `_physics_process()`.

## Phase 4: The Presentation Layer (Godot)
**Goal:** Godot becomes a dumb renderer.
- [ ] Remove `move_and_slide()` and `delta-rollback` SyncManager calls from GDScript.
- [ ] In `_physics_process()`, poll local input, send to Rust, and then update sprite `position` based on what Rust returns.
- [ ] Use `match get_p1_action():` to trigger the correct animations in Godot's `AnimationPlayer`.

## Phase 5: Network Transport
**Goal:** Replace SyncManager's network handling.
- [ ] Set up Godot's `ENetMultiplayerPeer` (or raw Rust UDP sockets).
- [ ] Serialize inputs and frame numbers to a byte array and send over the network.
- [ ] Feed incoming network bytes into Rust's `process_network_input()` to trigger rollbacks.