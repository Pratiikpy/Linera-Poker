# Quick Start Guide

Get Linera Poker running in under 5 minutes.

## Prerequisites

- Rust toolchain (1.75+)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Linera CLI: See [Linera documentation](https://linera.dev/getting_started/installation.html)
- Node.js 18+

## Quick Deploy

```bash
# Clone and navigate
git clone https://github.com/linera-poker/linera-poker
cd linera-poker

# Build contracts
cargo build --release --target wasm32-unknown-unknown

# Deploy contracts (takes ~2 minutes)
cd deploy && ./deploy.bash

# Start Linera service (in a new terminal)
linera service --port 8080

# Start frontend (in another terminal)
cd frontend
npm install
npm run dev
```

Open `http://localhost:5173` in **two browser windows** side-by-side to play.

## Playing Your First Game

### Step 1: Join Table
1. **Window 1**: Click "Join Table" for Player A
2. **Window 2**: Click "Join Table" for Player B
3. Both players stake tokens on their respective chains

### Step 2: Cards Are Dealt
- Each window shows ONLY that player's hole cards
- **Privacy**: Player B cannot see Player A's cards (different chain!)
- Community cards appear on the table

### Step 3: Betting Round
1. Player A (Window 1) - Raise, Call, Check, or Fold
2. Player B (Window 2) - Respond to the action
3. Continue through PreFlop, Flop, Turn, River

### Step 4: Showdown
- After final betting, both players reveal cards
- Cards are sent cross-chain to the table
- Winner determined by hand evaluation

### Step 5: Settlement
- Winner receives the pot on their chain
- Game resets for next hand

## Architecture Overview

```
┌────────────────────────────────────────────────────────────┐
│                    TABLE CHAIN (Dealer)                    │
│  • Game lifecycle management                               │
│  • Pot escrow and settlement                              │
│  • CANNOT see player hole cards                           │
└──────────────┬─────────────────────────┬──────────────────┘
               │                         │
               │  Cross-chain messages   │
               │                         │
      ┌────────▼─────────┐      ┌────────▼─────────┐
      │ PLAYER A CHAIN   │      │ PLAYER B CHAIN   │
      │ • Hole cards     │      │ • Hole cards     │
      │ • Token balance  │      │ • Token balance  │
      │ (PRIVATE!)       │      │ (PRIVATE!)       │
      └──────────────────┘      └──────────────────┘
```

## Key Features

| Feature | Description |
|---------|-------------|
| **Architectural Privacy** | Cards stored on player chains - dealer cannot access |
| **No Orchestrator** | Pure Linera SDK, no backend servers |
| **Authenticated Messages** | All cross-chain communication cryptographically signed |
| **True Ownership** | Your chips live on YOUR chain |

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release --target wasm32-unknown-unknown
```

### Deployment Issues

```bash
# Reset deployment state
cd deploy && ./deploy.bash --clean
```

### Connection Issues

```bash
# Check Linera service
curl http://localhost:8080/health

# Verify chains exist
linera query-chains
```

### Version Compatibility

Ensure your Linera CLI is compatible with SDK 0.15:
```bash
linera --version
```

## Test Commands

```bash
# Check deployment status
cat deploy/.deploy_state

# View contract logs
linera service --port 8080 2>&1 | grep -i poker

# Query chain state
linera query-application --chain <CHAIN_ID>
```

## Next Steps

- [Architecture Deep Dive](ARCHITECTURE.md) - Technical design details
- [README](README.md) - Project overview and roadmap
- [Terms of Service](legal/TERMS.md) - Usage terms

## Support

Having issues? [Open a GitHub Issue](https://github.com/linera-poker/linera-poker/issues)

---

**Linera Poker** - Your cards, your chain, your game.
