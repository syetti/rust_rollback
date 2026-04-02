# rust_rollback

Hyper-specialized Rust rollback core for a Godot fighting game.

## Overview

This project is a Rust + Godot (gdext) rollback prototype focused on deterministic state stepping and fast resimulation.

Current codebase includes:
- A fixed-size rollback buffer (`MAX_ROLLBACK_FRAMES = 60`)
- Circular history for game states and both players' inputs
- Late remote input handling with rollback + replay
- A Godot `Node` class (`FgSession`) exposed through GDExtension

## Project Structure

- `src/state.rs`: Core deterministic state types (`GameState`, `PlayerState`, `PlayerInput`)
- `src/rollback_core.rs`: Rollback engine (`RollbackCore`) and frame processing logic
- `src/lib.rs`: Godot integration (`FgSession`) and exported methods
- `src/main.rs`: Placeholder binary entrypoint (`Hello, world!`)

## How It Works

### 1) Frame Advance

`RollbackCore::advance_frame()`:
- Reads current frame state from circular buffer
- Reads frame inputs from input buffers
- Runs simulation step (placeholder in current implementation)
- Increments frame counters
- Stores the resulting state back into history

### 2) Remote Input Arrival

`RollbackCore::process_network_input(remote_frame, remote_input)`:
- Inserts newly arrived remote input into `p2_inputs`
- If input belongs to a past frame, triggers rollback

### 3) Rollback + Resimulation

`RollbackCore::rollback_to(frame)`:
- Jumps `current_frame` back to `frame`
- Replays frames up to present via repeated `advance_frame()`

## Godot Integration

`FgSession` in `src/lib.rs` is exported as a Godot class:
- `physics_process()` advances one simulation frame per physics tick
- `get_p1_x()` exposes Player 1 X position for rendering/inspection
- `receive_network_packet(frame, input)` feeds remote inputs into rollback core

## Build

### Prerequisites

- Rust toolchain (stable)
- Godot 4.x
- `godot` crate dependency (already set in `Cargo.toml`)

### Compile

```bash
cargo build
```

For optimized production build:

```bash
cargo build --release
```

The library is configured as:

```toml
[lib]
crate-type = ["cdylib"]
```

so the compiled artifact can be loaded by a Godot GDExtension setup.

## Important Notes

- Determinism is critical for rollback. Keep simulation pure and deterministic.
- Current simulation step is a placeholder; you should replace it with game logic.
- `confirmed_frame` exists but is not fully advanced/managed yet.
- `p1_inputs` local prediction/collection is not implemented yet.

## Suggested Next Steps

1. Implement deterministic `simulate(state, p1_input, p2_input)` function.
2. Add local input capture and prediction strategy for missing remote frames.
3. Track and advance `confirmed_frame` correctly.
4. Add tests for rollback correctness and state consistency.
5. Add serialization (for networking/debug snapshots), e.g. with `serde` + `bincode`.

## License

No license file is currently included.
Add a `LICENSE` file before distributing this project.
