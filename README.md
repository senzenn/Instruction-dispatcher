# Instruction Dispatcher

A Solana program that implements a counter with instruction dispatching and custom error handling.

## Overview

This program demonstrates:
- Instruction enum with 6+ operations
- Dispatcher pattern using match statements
- Custom error types with proper mapping to ProgramError
- Clear logging with `msg!` macros
- **Instruction Dispatching** - Master the dispatcher pattern
- **State Management** - Learn Solana account state handling
- **Error Handling** - Understand custom program errors
- **Program Architecture** - Build maintainable Solana programs

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
cargo build-sbf
```

## Testing

```bash
cargo test
```

## Deployment

### Prerequisites
- Install Solana CLI: `sh -c "$(curl -sSfL https://release.solana.com/v1.18.11/install)"`
- Set up devnet: `solana config set --url https://api.devnet.solana.com`

### Deploy to Devnet
```bash
# Build the program
cargo build-sbf

# Deploy to devnet
solana program deploy target/deploy/instruction_dispatcher.so

# Get program ID for testing
solana-keygen pubkey target/deploy/instruction_dispatcher-keypair.json
```

### Program ID
The deployed program ID is: `6Sxeh1AVCdt5MHwgqU9mhbFENmJLPfcHr9JpLnXKuGUi`

### Testing Your Deployed Program

Since your program is now deployed to devnet, you can interact with it using:

**Option 1: Create a Client Application**
```rust
// Example Rust client code
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

const PROGRAM_ID: &str = "6Sxeh1AVCdt5MHwgqU9mhbFENmJLPfcHr9JpLnXKuGUi";

// Initialize counter
let instruction_data = vec![0]; // Initialize instruction
let instruction = Instruction::new_with_bytes(
    Pubkey::from_str(PROGRAM_ID)?,
    &instruction_data,
    vec![AccountMeta::new(counter_account.pubkey(), false)],
);

// Send transaction...
```

**Option 2: Use Solana Playground**
- Visit https://beta.solana.com/playground
- Create a new project and import your program ID
- Write JavaScript/TypeScript code to interact with your program


### Manual Testing Commands (Reference)
The CLI commands below show the instruction format (you'll need a client app to execute them):

```bash
# Initialize counter (instruction 0)
# Accounts: [writable counter_account]
# Data: [0]

# Increment counter (instruction 1)
# Accounts: [writable counter_account]
# Data: [1, value_bytes...]

# Decrement counter (instruction 2)
# Accounts: [writable counter_account]
# Data: [2, value_bytes...]

# Reset counter (instruction 3)
# Accounts: [writable counter_account]
# Data: [3]

# Set counter (instruction 5)
# Accounts: [writable counter_account]
# Data: [5, value_bytes...]

# NoOp (instruction 6)
# Accounts: [writable counter_account]
# Data: [6]
```

## State

```rust
pub struct Counter {
    pub value: u64,
}
```