# Linera Poker - Wave 5 Demo Instructions

**FOR JUDGES: Single-Command Setup**

This demo shows **cross-chain mental poker** where each player's cards are on their own microchain.
The dealer literally CANNOT see player cards - architectural privacy!

---

## Quick Start (One Command)

### Prerequisites
- Docker and Docker Compose installed
- Modern browser (Chrome/Edge recommended)

### Step 1: Start Everything

```bash
# Clone the repository (if not already done)
git clone <repository-url>
cd linera-poker

# Start all services with Docker
docker compose up --build
```

**First run takes ~5-10 minutes** (builds Rust contracts, installs dependencies).

**Expected output when ready:**
```
╔═══════════════════════════════════════════════════════════════╗
║                     DEPLOYMENT COMPLETE                        ║
╚═══════════════════════════════════════════════════════════════╝

   Frontend:       http://localhost:5173
   Faucet:         http://localhost:8080
   GraphQL:        http://localhost:9001
   Validator:      http://localhost:13001

Ready to play poker on the Linera blockchain!
```

### Step 2: Open Browser & Play

1. **Open:** http://localhost:5173
2. **Connect Wallet:** Click "Connect Wallet" → Accept prompts
3. **Verify Connection:** Green badges appear showing "Connected"

---

## Playing Poker (Two Browser Windows)

### Setup
1. Open http://localhost:5173 in **Window 1** (Player A)
2. Open http://localhost:5173 in **Window 2** (Player B)

### Joining the Game
1. **Both windows:** Click "Connect Wallet"
2. **Window 1 (Player A):** Click "Create Table"
3. **Window 1:** Note the Table ID displayed
4. **Window 2 (Player B):** Enter Table ID → Click "Join Table"

### Playing a Hand
1. Both players see "Waiting for Players" → "Game Starting"
2. **Cards Dealt:** Each player sees ONLY their 2 hole cards
3. **Blinds:**
   - Player A (Small Blind): 5 chips automatically posted
   - Player B (Big Blind): 10 chips automatically posted
4. **Betting:** Use Check, Bet, or Fold buttons
5. **Community Cards:** Flop (3) → Turn (1) → River (1)
6. **Showdown:** Best hand wins the pot

### Privacy Verification
Open DevTools (F12) → Network tab to see:
- Queries go to DIFFERENT chains for each player
- Dealer chain NEVER receives hole card data
- Cross-chain messages only contain betting actions

---

## Port Reference

| Port | Service | Description |
|------|---------|-------------|
| 5173 | Frontend | Poker game UI (Vite dev server) |
| 8080 | Faucet | Token distribution for new chains |
| 9001 | GraphQL | Blockchain query service |
| 13001 | Validator | Linera network node |

---

## Key Innovation: Cross-Chain Privacy

### Traditional Poker (Ethereum):
```
┌─────────────────────────┐
│   Single Contract       │
│  - Dealer sees all cards│
│  - Players see all cards│
│  - TRUST REQUIRED       │
└─────────────────────────┘
```

### Linera Poker (This Project):
```
        ┌─────────────────┐
        │  TABLE CHAIN    │
        │  (Dealer)       │
        │  Cannot see     │
        │  player cards!  │
        └────────┬────────┘
                 │
     ┌───────────┴───────────┐
     │                       │
     ▼                       ▼
┌─────────┐            ┌─────────┐
│ PLAYER A│            │ PLAYER B│
│  CHAIN  │            │  CHAIN  │
│ PRIVATE │            │ PRIVATE │
│  cards  │            │  cards  │
└─────────┘            └─────────┘
```

**The dealer literally CANNOT cheat** - it's architecturally impossible!

---

## Contracts Deployed (Automatically)

When you run `docker compose up`, the following contracts are automatically built and deployed:

1. **Token Contract** - Manages chip balances
2. **Table Contract** - Game state machine, pot management
3. **Hand Contract (Player A)** - Player A's private cards
4. **Hand Contract (Player B)** - Player B's private cards

All deployment happens inside the Docker container. No manual setup required!

---

## Troubleshooting

### Container won't start
```bash
# Clean rebuild
docker compose down -v
docker compose build --no-cache
docker compose up
```

### Port already in use
```bash
# Windows
netstat -ano | findstr :5173
taskkill /PID <PID> /F

# Mac/Linux
lsof -ti:5173 | xargs kill -9
```

### View container logs
```bash
docker compose logs -f
```

### "Connecting..." won't change to "Connected"
- Check container is running: `docker compose ps`
- Check logs for errors: `docker compose logs -f`
- Hard refresh browser: Ctrl+Shift+R

### Wallet won't connect
- Try using Chrome or Edge
- Refresh page and try again
- Check browser console (F12) for errors

---

## Stopping the Demo

```bash
docker compose down
```

To remove all data (clean restart):
```bash
docker compose down -v
```

---

## Conway Testnet (Alternative)

If you prefer to test against the live Conway Testnet instead of local Docker:

1. Update `frontend/.env` with Conway configuration
2. Install Linera CLI: `cargo install linera-service@0.15.8`
3. Run: `linera service --port 9001`
4. Run frontend: `cd frontend && npm install && npm run dev`

**Note:** Conway Testnet requires internet access and may have higher latency.

---

## Buildathon Requirements Met

| Requirement | Status | Evidence |
|-------------|--------|----------|
| Docker compose template | ✅ | `Dockerfile`, `compose.yaml`, `run.bash` |
| Ports 5173, 8080, 9001, 13001 | ✅ | `compose.yaml` port mappings |
| Healthcheck on 5173 | ✅ | `Dockerfile` HEALTHCHECK directive |
| Linera SDK 0.15.8 | ✅ | `Dockerfile` + `Cargo.toml` |
| WASM contracts | ✅ | `table/`, `hand/`, `token/` directories |
| Frontend | ✅ | `frontend/` with React + TypeScript |
| Automatic deployment | ✅ | `run.bash` script |

---

## Technical Metrics

- **Cross-chain latency:** ~300ms message delivery
- **Contract sizes:**
  - Table: ~400KB WASM
  - Hand: ~200KB WASM
  - Token: ~150KB WASM
- **Hand evaluation:** 21 poker combinations supported
- **Frontend:** React 18 + Vite + Tailwind CSS

---

## Contact

Built for **Linera WaveHack Wave 5**

Questions? Check container logs with `docker compose logs -f` - all operations are logged for debugging!
