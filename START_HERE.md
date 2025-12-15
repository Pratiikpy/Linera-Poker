# Start Here - Linera Poker Evaluation Guide

Welcome to Linera Poker! This guide helps you navigate the documentation based on your available time.

## Quick Navigation

### I Have 5 Minutes
1. Read [QUICKSTART.md](./QUICKSTART.md)
   - 30-second demo instructions
   - Key innovations summary
   - What makes this special

### I Have 15 Minutes
1. Read [QUICKSTART.md](./QUICKSTART.md) (5 min)
2. Run the demo (10 min):
   ```bash
   cd deploy && ./deploy.bash
   linera service --port 8080  # in new terminal
   cd frontend && npm run dev  # in another terminal
   ```
3. Open http://localhost:5173 in two windows

### I Have 30 Minutes
1. [QUICKSTART.md](./QUICKSTART.md) - Quick demo (10 min)
2. [README.md](./README.md) - Full overview (10 min)
3. Review key contract code (10 min):
   - `table/src/contract.rs` - Game logic
   - `hand/src/contract.rs` - Private cards
   - `shared/src/lib.rs` - Hand evaluation

### I Have 1 Hour
1. [QUICKSTART.md](./QUICKSTART.md) (10 min)
2. [ARCHITECTURE.md](./ARCHITECTURE.md) - Technical deep-dive (30 min)
3. Play through full game cycle (10 min)
4. Review deployment automation (10 min):
   - `deploy/deploy.bash`
   - `docker-compose.yml`

## Documentation Map

```
linera-poker/
├─ START_HERE.md              ← You are here
├─ QUICKSTART.md               ← Start here for judges
├─ README.md                   ← Full project overview
├─ ARCHITECTURE.md             ← Technical deep-dive
├─ docker-compose.yml          ← Infrastructure as code
│
├─ table/src/contract.rs       ← Dealer game logic
├─ hand/src/contract.rs        ← Player card privacy
├─ shared/src/lib.rs           ← Hand evaluation
├─ frontend/src/App.tsx        ← React UI
└─ deploy/deploy.bash          ← Deployment automation
```

## The 30-Second Pitch

**On Ethereum**: All cards in one contract. Dealer can see everything. Trust required.

**On Linera**: Each player's cards on their OWN chain. Dealer literally CANNOT access them. Privacy is architectural, not cryptographic.

This is **impossible** on traditional blockchains.

## Key Innovation Summary

1. **Architectural Privacy**: Dealer chain cannot read player chain state (Linera runtime enforces this)
2. **No Orchestrator**: Pure Linera contracts, no external services
3. **Cross-Chain Messages**: All coordination via `send_to()` with authentication
4. **Blocking States**: Game cannot proceed without required messages
5. **Token Sovereignty**: Each player's chips on their own chain

## Quick Demo Commands

```bash
# Deploy (one-time setup)
cd deploy && ./deploy.bash

# Start Linera service
linera service --port 8080

# Start frontend (new terminal)
cd frontend && npm run dev

# Open two browser windows
# Window 1: Player A view
# Window 2: Player B view
# URL: http://localhost:5173
```

## Evaluation Checklist

- [ ] Read QUICKSTART.md
- [ ] Understand why this is impossible on Ethereum
- [ ] Run the demo successfully
- [ ] See cards are private per player
- [ ] Review contract code quality
- [ ] Assess documentation completeness
- [ ] Evaluate roadmap ambition

## Questions?

All common questions answered in:
- [QUICKSTART.md](./QUICKSTART.md) - Quick setup guide
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Technical details

## Judging Criteria Coverage

| Criterion | Score | Evidence |
|-----------|-------|----------|
| Innovation | 25% | ARCHITECTURE.md "Core Innovation" section |
| Technical | 25% | Contract code + ARCHITECTURE.md deep-dive |
| Usability | 20% | QUICKSTART.md + working demo |
| Completeness | 15% | Full game cycle + roadmap |
| Presentation | 15% | This documentation package |

---

**Thank you for judging Linera Poker!**

Built for Wave-6 Buildathon with ❤️
