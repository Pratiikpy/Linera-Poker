# Linera Poker  

**The first provably fair poker protocol. Your cards live on YOUR chain.**

[![Live on Conway](https://img.shields.io/badge/Live-Conway%20Testnet-green)](https://testnet-conway.linera.net)
[![Built on Linera](https://img.shields.io/badge/Built%20on-Linera-blue)](https://linera.io)
[![CI Status](https://img.shields.io/badge/CI-passing-brightgreen)](.github/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 🏆 Linera WaveHack Wave 5 Submission

> **📋 Changelog:** See [CHANGELOG.md](CHANGELOG.md) for all Wave 4 → Wave 5 improvements

### 🎯 For Judges: Docker Demo (Recommended)

**One command deploys everything:**
```bash
docker compose up --build
```

Wait 5-10 minutes for build, then open http://localhost:5173

**Why Docker?** Guarantees reproducible environment with all dependencies. See [RUN_DEMO.md](RUN_DEMO.md) for complete walkthrough.

### 🌐 Live Preview (Conway Testnet)

**Preview URL:** https://linera-poker-conway.netlify.app

> ⚠️ **Note:** The Netlify deployment is a **preview only**. Due to Conway Testnet CORS limitations, full gameplay requires the Docker demo above. The Netlify preview demonstrates:
> - ✅ Professional UI with loading animations
> - ✅ Wallet connection flow
> - ⚠️ Game state queries blocked by CORS (infrastructure limitation)

### 🎯 Key Buildathon Features

| Requirement | Implementation | Evidence |
|-------------|----------------|----------|
| **Docker Template** | ✅ Dockerfile + compose.yaml | Ports 5173, 8080, 9001, 13001 |
| **@linera/client Usage** | ✅ `useLineraWallet` hook | `frontend/src/hooks/useLineraWallet.ts` |
| **Browser-Based** | ✅ No CLI needed | Run Docker and open browser |
| **Linera SDK 0.15.8** | ✅ Latest version | `Dockerfile:34` |
| **Complete Demo** | ✅ One command | `docker compose up --build` |

**Time to verify:** < 10 minutes (see [JUDGING.md](JUDGING.md))

### ⚡ Performance Highlights

| Metric | Value | Industry Standard |
|--------|-------|-------------------|
| **Conway Connection Time** | 2.5s | N/A (unique to Linera) |
| **Cross-Chain Latency** | 180ms | 500ms (Ethereum L2) |
| **Contract Size** | 655 KB total | < 1 MB limit |
| **Frontend Load Time** | 1.2s (FCP) | < 2s target |

**Full benchmarks:** [PERFORMANCE.md](PERFORMANCE.md)

---

## Conway Testnet Deployment

**Contracts deployed and verified on Conway Testnet - December 15, 2025**

| Component | Chain ID | App ID |
|-----------|----------|--------|
| **Table (Dealer)** | `785ec7fcb1e9d2e71ecb96238de4e675925a8b93a8a44da187e7f9d88e3a5852` | `972b9df7ede594a4809e36bdda162a8ccf768d7f927759cc12473cdacbc0db09` |
| **Player A Hand** | `0a946b4759b993db660867f58cd7ec3b1b927d574274ede324ac6d6faeefe735` | `07f31116244dad0e56876141fbaa48ddf4dd53131694b821a2859f412c4d4af7` |
| **Player B Hand** | `545c9703f298c608e8543afa86bf1509c0d242ad0aed8d255ab6762d18bc81d3` | `9380fea81957b433034fcf2f20ba0a46622f156f14167651fc767d9a31cb4f49` |

### 🎬 Demo Video

[![Linera Poker Demo](https://img.shields.io/badge/Demo-Watch%20Now-red)](https://youtu.be/xoGuE8tNBq0?si=OK5mAzOMQnOPrSQt)

https://youtu.be/xoGuE8tNBq0?si=OK5mAzOMQnOPrSQt

---


## The Problem

Online poker is a **$60B+ market** plagued by a fundamental trust problem:

- Players must trust operators not to peek at cards
- Centralized servers can be compromised or manipulated
- No way to verify fairness without trusting the house
- Collusion detection relies on operator honesty

**Every existing solution requires trusting someone with your cards.**

## Our Solution

Linera Poker uses **cross-chain architecture** to make cheating **architecturally impossible**:

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  DEALER CHAIN   │     │  PLAYER A CHAIN │     │  PLAYER B CHAIN │
│  (Table State)  │     │  (A's Cards)    │     │  (B's Cards)    │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ - Game phase    │     │ - Hole cards    │     │ - Hole cards    │
│ - Pot amount    │     │ - Bet history   │     │ - Bet history   │
│ - Community     │     │ - Token balance │     │ - Token balance │
│   cards         │     │ (PRIVATE!)      │     │ (PRIVATE!)      │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         └───────────────────────┴───────────────────────┘
                    Cross-Chain Messages ONLY
                    (Dealer CANNOT access player state)
```

**Your cards are on YOUR chain. The dealer literally cannot see them.**

## Why This Matters

| Traditional Online Poker | Linera Poker |
|-------------------------|--------------|
| Cards stored on operator server | Cards on player's own chain |
| Trust the house | Trustless by design |
| Can be hacked/manipulated | Cryptographically secured |
| Opaque fairness claims | Verifiable on-chain |
| Centralized control | Player sovereignty |

## How It Works

### Mental Poker Protocol on Microchains

1. **Join**: Players stake tokens on their own chains
2. **Deal**: Dealer sends encrypted cards cross-chain (can't see contents)
3. **Bet**: Actions flow as authenticated cross-chain messages
4. **Reveal**: Players reveal cards only at showdown
5. **Settle**: Winner receives pot automatically

The game **cannot determine a winner** until both players reveal cards cross-chain. There is no bypass. The protocol enforces fairness.

## Technical Architecture

### Smart Contracts

| Contract | Location | Purpose |
|----------|----------|---------|
| **TableContract** | Dealer Chain | Game lifecycle, pot escrow, winner determination |
| **HandContract** | Player Chain | Private cards, betting actions |
| **TokenContract** | Player Chain | Chip balances, stake management |

### Key Features

- **Pure Linera SDK 0.15**: No orchestrator, no external services
- **Native Cross-Chain**: Uses `send_to()` for all inter-chain communication
- **Message Authentication**: `with_authentication()` on all messages
- **Blocking States**: Game cannot proceed without required messages
- **Per-User Token Sovereignty**: Your chips live on YOUR chain

## Business Model

Linera Poker is designed for sustainable operation:

| Revenue Stream | Rate | Industry Standard |
|---------------|------|-------------------|
| **Rake** | 2.5% of pot | 2.5-10% |
| **Tournament Fees** | 10% of buy-in | 10-15% |
| **Premium Tables** | Subscription | N/A |

Projected addressable market: $60B+ annually (online poker industry)

## Getting Started

### Prerequisites

- Rust toolchain (1.75+)
- `wasm32-unknown-unknown` target
- Linera CLI
- Node.js 18+

### Quick Start

```bash
# Clone the repository
git clone https://github.com/linera-poker/linera-poker

# Build contracts
cargo build --release --target wasm32-unknown-unknown

# Deploy (local development)
cd deploy && ./deploy.bash

# Start frontend
cd frontend && npm install && npm run dev
```

See [QUICKSTART.md](QUICKSTART.md) for detailed setup instructions.

## Roadmap

### Phase 1: Core Game (Current - Private Beta)
- Two-player Texas Hold'em
- Cross-chain privacy for hole cards
- Real-time betting rounds
- Provably fair showdown

### Phase 2: Multi-Table Support ()
- Multiple concurrent tables
- Variable stake levels
- Table discovery and lobbies
- Spectator mode

### Phase 3: Tournament Features ()
- Sit-and-go tournaments
- Multi-table tournaments (MTT)
- Prize pool distribution
- Blind level progression

### Phase 4: Mobile & Social ()
- Progressive web app (PWA)
- Friend invites and private tables
- Player profiles and achievements
- Cross-chain leaderboards

### Phase 5: DAO Governance ()
- Decentralized rake management
- Community voting on game rules
- Revenue sharing for token holders
- Protocol upgrades via proposals

## Security

### What the Dealer CAN See
- Game phase, pot amount, community cards
- Which players have folded
- Bet amounts

### What the Dealer CANNOT See
- Player hole cards (stored on player chains)
- Player reveal keys
- Player strategy

### Responsible Gaming

We are committed to responsible gaming practices. See our [Responsible Gaming Policy](legal/RESPONSIBLE_GAMING.md).

## Documentation

### For Judges
- **[JUDGING.md](JUDGING.md)** - 2-minute verification guide
- **[PERFORMANCE.md](PERFORMANCE.md)** - Comprehensive benchmarks
- **[docs/WHY_AMBITIOUS.md](docs/WHY_AMBITIOUS.md)** - Technical complexity explained

### For Developers
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Architecture deep dive
- **[QUICKSTART.md](QUICKSTART.md)** - Quick start guide
- **[docs/DEMO_GUIDE.md](docs/DEMO_GUIDE.md)** - Demo recording guide

## Legal

- [Terms of Service](legal/TERMS.md)
- [Privacy Policy](legal/PRIVACY.md)
- [Responsible Gaming](legal/RESPONSIBLE_GAMING.md)



## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Linera Poker** - Provably fair poker. Your cards, your chain, your game.

*Featured in Linera Wave Buildathon*





