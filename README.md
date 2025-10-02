# Instruction Dispatcher

A Solana program that implements a counter with instruction dispatching and custom error handling.

## Overview

This program demonstrates:
- Instruction enum with 6+ operations
- Dispatcher pattern using match statements
- Custom error types with proper mapping to ProgramError
- Clear logging with `msg!` macros

## Instructions

- `Initialize` - Initialize counter to 0
- `Increment(u64)` - Add value to counter (prevents overflow)
- `Decrement(u64)` - Subtract value from counter (prevents underflow)
- `Reset` - Reset counter to 0
- `Set(u64)` - Set counter to specific value (max 100)
- `NoOp` - No operation

## Custom Errors

- `MaxValue` - Counter would exceed maximum value
- `Underflow` - Counter would go below 0
- `InvalidValue` - Invalid value provided (e.g., Set > 100)

## Building

```bash
cargo build-bpf
```

## Testing

```bash
cargo test
```

## State

```rust
pub struct Counter {
    pub value: u64,
}
```