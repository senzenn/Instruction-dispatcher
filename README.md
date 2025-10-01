# Instruction-dispatcher

## ---  Goal/ Project summary 

### Instruction Dispatcher & Error Handling

- **Description:** Program with 6+ instructions, solid dispatcher, custom errors, and clear `msg!` logs.
- **What to implement:** Enum for instructions, matching dispatcher, custom `ProgramError` mapping.
- **Deliverables:** Instruction tests and a client that demonstrates error cases.



                                        --------------------------------+      +--------------------------------+      +---------------------------------+
                                        |      PART 1: OFF-CHAIN         |      |    PART 2: SOLANA NETWORK      |      |      PART 3: ON-CHAIN           |
                                        |         (Your Computer)        |      |         (The Blockchain)       |      |    (Your Rust Program Code)     |
                                        +--------------------------------+      +--------------------------------+      +---------------------------------+
                                        |                                |      |                                |      |                                 |
                                        | [ Web App / JS Script ]        |      |                                |      |  [ entrypoint.rs ]              |
                                        |   - User clicks "Increment"    |      |                                |      |    - The single door in         |
                                        |   - Uses @solana/web3.js       |      |                                |      |                                 |
                                        |   - Uses borsh-js to           |      |                                |      |             +-------------------> [ processor.rs ]
                                        |     "serialize" instruction    |      |                                |      |             |                     - The "Brain" / Dispatcher
                                        |     into bytes                 |      |                                |      |             |                     - Deserializes bytes
                                        |                                |      |                                |      |             |                     - `match` statement calls
                                        |             |                  |      |                                |      |             |                       the right function
                                        |             |                  |      |                                |      |             |
                                        |             v                  |      |                                |      |             |
                                        |  [ Transaction ]               |      |                                |      |             |
                                        |   - Contains program ID        |      |                                |      |             v
                                        |   - Contains accounts          |      |                                |      |       [ Logic Functions ]
                                        |   - Contains the byte data     |      |                                |      |         - process_increment()
                                        |                                |      |                                |      |         - process_set()
                                        |             |                  |      |                                |      |           - Reads Account Data
                                        |             |  sends           |      |                                |      |       - Applies logic
                                        |             +------------------> [ RPC Node ] --------> [ Runtime ] --+---------> + - Writes back Account Data
                                        |                                |   - Gateway to Solana    - Executes   |      |                                 |
                                        |                                |                          your code    |      |                                 |
                                        |             ^                  |      |                                |      |                                 |
                                        |             | confirms         |      |                                |      |                                 |
                                        |             +------------------+                                       |      |                                 |
                                        |                                |      |                                |      |                                 |
                                        +--------------------------------+      +--------------------------------+      +---------------------------------+



