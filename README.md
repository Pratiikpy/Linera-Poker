# Linera Poker

**The first provably fair poker protocol. Your cards live on YOUR chain.**

**[Live Demo](https://linera-poker.vercel.app)** | **Docker:** `docker compose up --build`

[![Built on Linera](https://img.shields.io/badge/Built%20on-Linera-blue)](https://linera.io)
[![Linera SDK](https://img.shields.io/badge/SDK-0.15.8-green)](https://crates.io/crates/linera-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## Quick Start

**Live Demo:** https://linera-poker.vercel.app

**Docker (local with blockchain backend):**
```bash
docker compose up --build
```

Open http://localhost:5173 after build completes. **No wallet extension needed** - auto-connects to Linera faucet.

---

## Linera WaveHack Wave 6 Submission

> **Changelog:** See [CHANGELOG.md](CHANGELOG.md) for Wave 5 to Wave 6 improvements

### Demo Video

[![Linera Poker Demo](https://img.shields.io/badge/Demo-Watch%20Now-red)](https://youtu.be/xoGuE8tNBq0?si=OK5mAzOMQnOPrSQt)

https://youtu.be/xoGuE8tNBq0?si=OK5mAzOMQnOPrSQt

### For Judges: 2-Minute Verification

**After `docker compose up --build` completes:**

1. Open http://localhost:5173 - should auto-connect (no wallet prompt)
2. Click "ENTER THE TABLE"
3. Join as Player A (100 chips) and Player B (100 chips)
4. Play through betting rounds
5. Verify via GraphQL:

```bash
# Table state
curl -s -X POST http://localhost:8081/chains/${TABLE_CHAIN}/applications/${TABLE_APP} \
  -H "Content-Type: application/json" \
  -d '{"query":"{ state { gameId phase pot turnSeat communityCards { suit rank } } }"}' | python3 -m json.tool

# Player A hand (private chain)
curl -s -X POST http://localhost:8081/chains/${PLAYER_A_CHAIN}/applications/${PLAYER_A_APP} \
  -H "Content-Type: application/json" \
  -d '{"query":"{ state { gameId seat holeCards { suit rank } myTurn } }"}' | python3 -m json.tool
```

See [BUILDATHON_SUBMISSION.md](BUILDATHON_SUBMISSION.md) for detailed verification.

### What Makes This Unique: Cross-Chain Card Privacy

```
        +-----------------+
        |  TABLE CHAIN    |
        |  (Dealer)       |
        |  Cannot see     |
        |  player cards!  |
        +--------+--------+
                 |
     +-----------+-----------+
     |                       |
     v                       v
+---------+            +---------+
| PLAYER A|            | PLAYER B|
|  CHAIN  |            |  CHAIN  |
| PRIVATE |            | PRIVATE |
|  cards  |            |  cards  |
+---------+            +---------+
```

**The dealer literally CANNOT cheat** - player cards are on separate microchains that the dealer chain has no read access to. This is architecturally impossible on single-chain systems (Ethereum, Solana).

### Buildathon Requirements

| Requirement | Status | Evidence |
|-------------|--------|----------|
| Docker Template | Done | `Dockerfile` + `compose.yaml` |
| @linera/client | Done | `frontend/src/contexts/WalletContext.tsx` - direct faucet pattern |
| Browser-Based | Done | No CLI needed, auto-connects |
| Linera SDK 0.15.8 | Done | `Cargo.toml` workspace deps |
| Complete Demo | Done | `docker compose up --build` |

### Key Wave 6 Change: Removed Dynamic Labs

Wave 5 used Dynamic Labs EVM wallet as a bridge to Linera. Wave 6 uses **direct `@linera/client`** faucet pattern matching winning projects (MicroSkribbl, Battleship):

```typescript
// WalletContext.tsx - Direct @linera/client usage
import { initialize, Faucet, Client, signer } from '@linera/client'

await initialize()
const privateKey = signer.PrivateKey.createRandom()
const faucet = new Faucet(FAUCET_URL)
const wallet = await faucet.createWallet()
const chainId = await faucet.claimChain(wallet, privateKey.address())
const client = new Client(wallet, privateKey)
const chain = await client.chain(chainId)
```

No external wallet needed. Auto-connects on page load.

---

## The Problem

Online poker is a **$60B+ market** plagued by a fundamental trust problem:

- Players must trust operators not to peek at cards
- Centralized servers can be compromised or manipulated
- No way to verify fairness without trusting the house

**Every existing solution requires trusting someone with your cards.**

## Our Solution

Linera Poker uses **cross-chain architecture** to make cheating **architecturally impossible**:

- **Your cards are on YOUR chain.** The dealer literally cannot see them.
- **Cross-chain messages only.** Betting actions flow between chains.
- **Reveal at showdown only.** Players control when cards are shared.

## Technical Architecture

### Smart Contracts

| Contract | Location | Purpose |
|----------|----------|---------|
| **TableContract** | `table/src/contract.rs` | Game lifecycle, pot escrow, winner determination |
| **HandContract** | `hand/src/contract.rs` | Private cards, betting actions |
| **TokenContract** | `token/src/contract.rs` | Chip balances, stake management |

### Key Features

- **Pure Linera SDK 0.15.8**: No orchestrator, no external services
- **Native Cross-Chain**: Uses `send_to()` for all inter-chain communication
- **Message Authentication**: `with_authentication()` on all messages
- **COOP/COEP Headers**: Proper SharedArrayBuffer support for WASM

## Getting Started

### Docker (Recommended)

```bash
docker compose up --build
# Open http://localhost:5173
```

### Manual Setup

```bash
# Prerequisites: Rust 1.86+, wasm32-unknown-unknown target, Node.js 22+

# Build contracts
cargo build --release --target wasm32-unknown-unknown

# Start local network
eval "$(linera net helper)"
linera_spawn linera net up --with-faucet

# Deploy (see run.bash for full deployment script)
```

## Documentation

- **[RUN_DEMO.md](RUN_DEMO.md)** - Complete demo walkthrough
- **[BUILDATHON_SUBMISSION.md](BUILDATHON_SUBMISSION.md)** - Buildathon submission details
- **[CHANGELOG.md](CHANGELOG.md)** - Wave 5 to Wave 6 changes
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Architecture deep dive

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Linera Poker** - Provably fair poker. Your cards, your chain, your game.

*Linera WaveHack Wave 6*
