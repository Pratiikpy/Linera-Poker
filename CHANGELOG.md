# Changelog

All notable changes to Linera Poker will be documented in this file.

---

## [Wave 6] - February 2026

### Breaking Changes

#### Removed Dynamic Labs Dependency
- **Removed:** `@dynamic-labs/ethereum` and `@dynamic-labs/sdk-react-core` from dependencies
- **Removed:** `frontend/src/lib/dynamic-signer.ts` (no longer needed)
- **Rewritten:** `WalletContext.tsx` - now uses direct `@linera/client` faucet pattern
- **Rewritten:** `useLineraWallet.ts` - simplified hook wrapping WalletContext
- **Rewritten:** `linera-adapter.ts` - removed DynamicSigner, simplified to utility class
- **Rewritten:** `App.tsx` - removed Dynamic Labs provider wrapper and EVM wallet prompt
- **Rewritten:** `main.tsx` - removed Dynamic Labs environment checks
- **Result:** Zero external wallet dependencies. Auto-connects to Conway Testnet faucet on page load.

### Improvements

#### Direct @linera/client Integration (Matches Winner Pattern)
- Auto-connects to Conway Testnet faucet on mount (no user action needed)
- Creates wallet, claims chain, creates client automatically
- Matches MicroSkribbl/Battleship pattern used by Wave 6 winners
- Updated `@linera/client` from `0.15.4` to `0.15.8`

#### Fixed Duplicate Polling
- Removed 3s auto-refresh from `App.tsx` (was doubling with `useGameState`'s 2s poll)
- Single 2s poll in `useGameState.ts` handles all state updates

#### Wave 6 Branding
- Updated all references from "Wave 5" to "Wave 6" across:
  - `compose.yaml`, `Dockerfile`, `run.bash`, `README.md`
  - `CHANGELOG.md`, `BUILDATHON.md`, `RUN_DEMO.md`, `JUDGING.md`

### Technical Details

| Change | Before (Wave 5) | After (Wave 6) |
|--------|-----------------|-----------------|
| Wallet | Dynamic Labs EVM + DynamicSigner bridge | Direct @linera/client faucet |
| @linera/client | 0.15.4 | 0.15.8 |
| Dependencies | 3 wallet packages | 0 wallet packages |
| Auto-connect | Required EVM wallet first | Auto on page load |
| Polling | 3s (App) + 2s (useGameState) | 2s (useGameState only) |

---

## [Wave 5] - December 2025

### 🎯 Wave 4 Feedback Response

This release directly addresses all feedback from Wave 4 judge @deuszx:

> "the live demo does not work fully... project should follow and use the docker compose template"

> "RUN_DEMO.md instructions are obviously incomplete – they don't mention building the code"

> "backend service is not configured to interact with any application/chain"

> "how are you planning to solve the problems of fair randomness? Or privacy of the players' hands?"

---

### ✅ Fixed Issues

#### 1. Docker Template Compliance
- **Added:** `Dockerfile` matching buildathon template with healthcheck
- **Added:** `compose.yaml` with required ports (5173, 8080, 9001, 13001)
- **Updated:** `run.bash` to be fully automated (507 lines)
- **Result:** Single command `docker compose up --build` deploys everything

#### 2. RUN_DEMO.md Completeness
- **Added:** "What Happens During `docker compose up --build`" section
- **Added:** Explicit contract compilation steps
- **Added:** Backend service configuration explanation
- **Added:** Environment file auto-generation documentation
- **Added:** Port reference table
- **Added:** Troubleshooting guide

#### 3. Backend Service Configuration
- **Clarified:** `linera service --port 9001` automatically reads wallet with all App IDs
- **Added:** Evidence of configuration in documentation
- **Added:** GraphQL endpoint examples for verification

#### 4. Randomness & Privacy Transparency
- **Created:** `JUDGE_RESPONSE.md` (518 lines) with honest assessment
- **Documented:** Current predictable seed limitation with code reference
- **Documented:** Commit-reveal scheme roadmap (2 weeks)
- **Documented:** Current public cards limitation
- **Documented:** ZK commitment roadmap (1 month)
- **Added:** "Acceptable Use Cases" section (demo ✅, real-money ❌)

---

### 📝 Documentation Updates

| File | Change |
|------|--------|
| `README.md` | Updated to direct judges to Docker demo first |
| `RUN_DEMO.md` | Complete rewrite with 7-stage build explanation |
| `JUDGE_RESPONSE.md` | NEW - 518 lines addressing all feedback |
| `JUDGING.md` | Updated verification steps |
| `Dockerfile` | NEW - Production Docker image |
| `compose.yaml` | NEW - Docker Compose configuration |

---

### 🔧 Technical Improvements

- **Dockerfile:** Rust 1.86 + Linera SDK 0.15.8 + Node.js 22
- **Healthcheck:** 5-minute start period for contract compilation
- **CRLF Handling:** Entrypoint converts Windows line endings
- **Error Handling:** run.bash uses `set -euo pipefail`
- **Logging:** Colored output with section banners

---

### 📊 Contract Status

| Contract | Lines | Status |
|----------|-------|--------|
| TableContract | 1,131 | ✅ Complete |
| HandContract | 500+ | ✅ Complete |
| TokenContract | 300+ | ✅ Complete |

---

### 🚀 What's Next (Post-Wave 5)

1. **Week 1-2:** Implement commit-reveal randomness
2. **Week 3-4:** Implement ZK card commitments
3. **Month 2:** Add timeout mechanisms
4. **Month 3:** Security audit

---

## [Wave 4] - December 2025

### Initial Submission
- Cross-chain poker game architecture
- Table, Hand, Token contracts
- React frontend with wallet integration
- Netlify deployment (with CORS limitations)

### Known Issues (Addressed in Wave 5)
- Docker template not followed
- RUN_DEMO.md incomplete
- Randomness not secure (demo only)
- Cards visible on-chain (demo only)

---

## [Wave 1-3] - October-November 2025

### Foundation
- Initial project setup
- Basic contract structure
- Frontend scaffolding
