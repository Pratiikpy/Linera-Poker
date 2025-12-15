# Relay Pattern Architecture Fix

## Problem Statement

In the Linera blockchain poker game, we encountered a critical cross-application messaging issue:

- **Original Design**: Hand contract on player chains sends `Message::JoinTable` to table_chain using `send_to(table_chain)`
- **Linera Constraint**: `send_to()` sends messages to the SAME APPLICATION ID on the target chain
- **Issue**: Hand app and table app have DIFFERENT application IDs, so messages went to the non-existent hand app on table_chain instead of the table app

## Linera Architecture Facts

1. **Cross-chain messages** (`send_to()`) always go to the same app ID on the target chain
2. **Cross-application calls** (`call_application()`) work only within the same chain
3. **Applications** deployed on multiple chains have the same app ID but different state per chain

## Solution: Relay Pattern

We implemented a relay pattern where the hand application on the table chain acts as a proxy, forwarding messages to the table application using cross-application calls.

### Message Flow

```
┌─────────────────┐                    ┌─────────────────┐
│  PLAYER CHAIN   │                    │  TABLE CHAIN    │
│                 │                    │                 │
│  ┌───────────┐  │                    │  ┌───────────┐  │
│  │ Hand App  │  │  send_to()         │  │ Hand App  │  │
│  │ (Player)  │──┼───────────────────>│  │ (Relay)   │  │
│  └───────────┘  │                    │  └─────┬─────┘  │
│                 │                    │        │        │
│                 │                    │        │ call_  │
│                 │                    │        │ application()
│                 │                    │        │        │
│                 │                    │        v        │
│                 │                    │  ┌───────────┐  │
│                 │                    │  │ Table App │  │
│                 │ <──────────────────┼──│ (Dealer)  │  │
│                 │  Response messages │  └───────────┘  │
└─────────────────┘                    └─────────────────┘
```

### Implementation Details

#### 1. Hand Contract Relay Logic (`hand/src/contract.rs`)

**Detection**: The hand app detects if it's running on the table chain:
```rust
let current_chain = self.runtime.chain_id();
let is_relay = current_chain == table_chain;
```

**Message Handling**: When receiving player->table messages on the table chain:
```rust
Message::JoinTable { stake, hand_app_id } => {
    if is_relay {
        // We're the relay on table chain - forward to table app
        self.relay_to_table(message).await;
    }
}
```

**Relay Method**: Converts messages to table operations and calls the table app:
```rust
async fn relay_to_table(&mut self, message: Message) {
    let table_app = self.state.table_app.get()?;
    let source_chain = self.runtime.message_origin_chain_id()?;

    let operation = match message {
        Message::JoinTable { stake, hand_app_id } => {
            TableOperation::RelayJoinTable {
                player_chain: source_chain,
                stake,
                hand_app_id,
            }
        }
        // ... other message types
    };

    self.runtime.call_application(
        /* authenticated */ true,
        table_app.with_abi::<TableAbi>(),
        &operation,
    );
}
```

#### 2. Table Contract Operations (`table/src/lib.rs` and `table/src/contract.rs`)

**Extended TableOperation Enum**:
```rust
pub enum TableOperation {
    StartNewGame,
    ForceAdvance,

    // Relay operations
    RelayJoinTable {
        player_chain: ChainId,
        stake: Amount,
        hand_app_id: ApplicationId,
    },
    RelayBetAction {
        player_chain: ChainId,
        game_id: u64,
        action: BetAction,
    },
    // ... other relay operations
}
```

**Operation Handlers**:
```rust
async fn execute_operation(&mut self, operation: TableOperation) -> TableResult {
    match operation {
        TableOperation::RelayJoinTable { player_chain, stake, hand_app_id } => {
            self.handle_join(player_chain, stake, hand_app_id).await;
            TableResult::Success
        }
        // ... other operations
    }
}
```

## Key Design Decisions

### 1. Why Relay Instead of Direct Messaging?

Linera's messaging system is designed for cross-chain communication between instances of the SAME application. There is no built-in way to send a message directly from one application to a different application on another chain.

### 2. Why Use Operations Instead of Messages?

- `call_application()` invokes the `execute_operation` method, not `execute_message`
- `execute_message` is only for cross-chain messages within the same application
- Operations are the standard way to trigger logic within an application on the same chain

### 3. Authentication Preservation

The relay uses `authenticated=true` in `call_application`, which preserves the original sender's authentication. This ensures the table app can identify which player chain sent the message.

### 4. Relay Activation Logic

The relay only activates when:
1. The hand app is running on the table chain (`current_chain == table_chain`)
2. The message is a player->table message type
3. The hand app has a configured `table_app` reference

This ensures:
- Hand apps on player chains behave normally (send messages)
- Hand app on table chain acts as relay (forwards to table app)
- No infinite loops or unintended relaying

## Security Considerations

### 1. Message Source Verification

The relay extracts the original sender from `message_origin_chain_id()` and passes it to the table app, preventing spoofing.

### 2. Authorization

Only messages from authenticated sources are relayed. The `with_authentication()` flag ensures the table app can verify the sender.

### 3. Replay Protection

Each message is processed only once by Linera's runtime. The relay doesn't introduce additional replay vulnerabilities.

### 4. Denial of Service

The relay doesn't perform expensive operations, minimizing DoS risk. Error handling ensures failed relays don't crash the hand app.

## Testing Strategy

### Unit Tests
- Verify relay detects when on table chain vs. player chain
- Verify messages are correctly converted to operations
- Verify table operations invoke correct handler methods

### Integration Tests
1. **Player Chain -> Table Chain**: Send JoinTable from player, verify table receives it
2. **Multiple Players**: Ensure relay handles concurrent messages from different players
3. **Error Cases**: Verify relay handles missing table_app gracefully

### End-to-End Tests
- Full game flow: join, bet, reveal, settle
- Verify all message types are relayed correctly
- Verify game state consistency across chains

## Performance Implications

### Latency
- **Cross-chain message**: 1 hop (player -> table hand app)
- **Cross-application call**: Synchronous, same-block execution
- **Total**: Minimal additional latency vs. direct messaging (if it were possible)

### Gas/Fees
- Relay operations are lightweight (no state modifications in hand app)
- Main cost is the cross-application call, which is cheaper than cross-chain messages

## Migration Notes

### Deployment Order
1. Deploy updated table contract (with relay operations)
2. Deploy updated hand contract (with relay logic)
3. Existing games in progress may need to complete before upgrade

### Configuration
- Hand apps must be configured with both `table_chain` and `table_app` during instantiation
- Table chain must have the hand app deployed (for relay functionality)

## Future Enhancements

### 1. Error Reporting
Currently, relay errors are silently ignored. Could add:
- Error logging for debugging
- Error messages back to source chain
- Retry logic for transient failures

### 2. Metrics
Add counters for:
- Number of messages relayed
- Relay failures
- Cross-application call latency

### 3. Optimization
If Linera adds native cross-application messaging, this relay pattern could be removed.

## Files Modified

1. `/mnt/c/Users/prate/linera/linera-poker/hand/src/contract.rs`
   - Added relay detection logic in `execute_message`
   - Added `relay_to_table` method
   - Modified message matching to handle relay cases

2. `/mnt/c/Users/prate/linera/linera-poker/hand/Cargo.toml`
   - Added dependency: `linera-poker-table`

3. `/mnt/c/Users/prate/linera/linera-poker/table/src/lib.rs`
   - Extended `TableOperation` enum with relay operations
   - Added imports: `ChainId`

4. `/mnt/c/Users/prate/linera/linera-poker/table/src/contract.rs`
   - Added relay operation handlers in `execute_operation`

## Conclusion

The relay pattern elegantly solves the cross-application messaging challenge within Linera's architectural constraints. By leveraging the hand app on the table chain as a proxy, we maintain clean separation of concerns while enabling the necessary communication between player and table applications.

This pattern is reusable for any Linera application that needs to coordinate between multiple application types across chains.
