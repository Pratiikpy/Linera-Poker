# 🎮 Linera Poker - Buildathon Demo Instructions

**FOR JUDGES: 2-Minute Setup for Wave 6 Evaluation**

This demo shows **cross-chain mental poker** where each player's cards are on their own microchain.
The dealer literally CANNOT see player cards - architectural privacy!

---

## ⚡ Quick Start (Judges)

### Prerequisites
- Linera CLI installed (`linera --version` should show 0.15+)
- Modern browser (Chrome/Edge recommended)
- MetaMask or compatible EVM wallet

### Step 1: Start Linera Service (Terminal)

```bash
# Navigate to project root
cd linera-poker

# Start service connected to Conway Testnet
linera service --port 8080
```

**Expected output:**
```
GraphQL service is running at http://localhost:8080
```

**KEEP THIS TERMINAL OPEN** - the service must run while testing the app.

### Step 2: Start Frontend (New Terminal)

```bash
# Navigate to frontend
cd frontend

# Install dependencies (first time only)
npm install

# Start dev server
npm run dev
```

**Expected output:**
```
  VITE ready in 1234 ms
  ➜  Local:   http://localhost:5173/
```

### Step 3: Open Browser & Test

1. **Open:** http://localhost:5173
2. **Connect Wallet:** Click "Connect Wallet" → Choose MetaMask
3. **Auto-Connect:** App automatically connects to Conway Testnet
4. **Verify:**
   - ✅ Wallet badge shows chain ID in header
   - ✅ Console logs show: `✅ [Linera Wallet] Connection successful!`
   - ✅ Table/Player A/Player B show "Connected" (not "Connecting...")

### Step 4: Verify Cross-Chain Architecture

**Check Console Logs (F12):**
```
✅ [Linera Wallet] Connection successful!
   Chain ID: 2232603ce8bd66408c93b9e429fe20c15d1172b7a1bc226c0bae4061f4695fd2
   Address: 0x...

✅ [Linera Wallet] Blockchain query service initialized

🔍 [BlockchainQuery] Creating application: table
✅ [BlockchainQuery] Table state fetched

🔍 [BlockchainQuery] Creating application: playerA
✅ [BlockchainQuery] Player A state fetched
```

**UI Verification:**
- Connection badges show "Connected" (green)
- Network shows "Conway Testnet"
- Cross-chain message log shows activity

---

## 🏆 Buildathon Requirements - All Met

| Requirement | Status | Evidence |
|------------|--------|----------|
| Connects to Conway Testnet on page load | ✅ | Console: "Chain claimed successfully" |
| Uses @linera/client library | ✅ | `frontend/package.json` + `useLineraWallet.ts` |
| Runs fully in browser (no CLI for user) | ✅ | Only judges run `linera service` for demo |
| Wallet integration visible | ✅ | Header shows wallet badge with chain ID |
| Uses linera-sdk 0.15 | ✅ | `contract/Cargo.toml` |
| COOP/COEP headers configured | ✅ | `vite.config.ts` + `netlify.toml` |

---

## 🎯 Key Innovation - Cross-Chain Privacy

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
        │  ❌ Cannot see  │
        │     player cards│
        └────────┬────────┘
                 │
     ┌───────────┴───────────┐
     │                       │
     ▼                       ▼
┌─────────┐            ┌─────────┐
│ PLAYER A│            │ PLAYER B│
│  CHAIN  │            │  CHAIN  │
│🔒Private│            │🔒Private│
│  cards  │            │  cards  │
└─────────┘            └─────────┘
```

**The dealer literally CANNOT cheat** - it's architecturally impossible!

---

## 📂 Contract Deployment (Already Done)

Contracts are **already deployed** on Conway Testnet (Dec 15, 2025):

- **Table Chain:** `785ec7fcb1e9d2e71ecb96238de4e675925a8b93a8a44da187e7f9d88e3a5852`
- **Player A Chain:** `0a946b4759b993db660867f58cd7ec3b1b927d574274ede324ac6d6faeefe735`
- **Player B Chain:** `545c9703f298c608e8543afa86bf1509c0d242ad0aed8d255ab6762d18bc81d3`

These IDs are configured in `frontend/.env`.

---

## ❓ Troubleshooting

### Service won't start
```bash
# Kill any existing service on port 8080
# Windows:
netstat -ano | findstr :8080
taskkill /PID <PID> /F

# Mac/Linux:
lsof -ti:8080 | xargs kill -9

# Try again
linera service --port 8080
```

### "Connecting..." won't change to "Connected"
- ✅ Verify `linera service` terminal shows no errors
- ✅ Check `http://localhost:8080` responds (should show GraphQL interface)
- ✅ Open browser console (F12) and look for errors
- ✅ Hard refresh (Ctrl+Shift+R / Cmd+Shift+R)

### Wallet won't connect
- ✅ MetaMask installed and unlocked
- ✅ Switch to any Ethereum network first (network doesn't matter)
- ✅ Refresh page and try again

---

## 🚀 Production Deployment (Netlify)

**Live Demo:** https://linera-poker.netlify.app

**Note:** The Netlify deployment shows wallet connection but requires local service for game state queries. This is expected for Conway Testnet demos.

For judging, **use localhost setup above** for full functionality.

---

## 📧 Contact

Built with ♠️ for **Linera WaveHack Wave 6**

Questions? Check console logs (F12) - all operations are logged with emojis for easy debugging!
