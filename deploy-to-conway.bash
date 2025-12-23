#!/bin/bash

################################################################################
# Linera Poker - Conway Testnet Deployment Script
# Deploys contracts to Conway testnet and prepares frontend for Netlify
################################################################################

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }

# Conway Testnet Configuration
FAUCET_URL="https://faucet.testnet-conway.linera.net"
SERVICE_URL="https://indexer.testnet-conway.linera.net"

# Wallet paths (will be created in user's home directory)
export LINERA_WALLET="$HOME/.config/linera-poker/wallet.json"
export LINERA_KEYSTORE="$HOME/.config/linera-poker/keystore.json"
export LINERA_STORAGE="rocksdb:$HOME/.config/linera-poker/client.db"

# Contract paths
WASM_DIR="target/wasm32-unknown-unknown/release"
TABLE_CONTRACT="${WASM_DIR}/table_contract.wasm"
TABLE_SERVICE="${WASM_DIR}/table_service.wasm"
HAND_CONTRACT="${WASM_DIR}/hand_contract.wasm"
HAND_SERVICE="${WASM_DIR}/hand_service.wasm"

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║        Linera Poker - Conway Testnet Deployment               ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

################################################################################
# Step 1: Build Contracts
################################################################################

log_info "Step 1/6: Building WASM contracts..."
if [ ! -f "${TABLE_CONTRACT}" ] || [ ! -f "${HAND_CONTRACT}" ]; then
    log_info "Contracts not found, building..."
    cargo build --release --target wasm32-unknown-unknown
    log_success "Contracts built successfully!"
else
    log_success "Contracts already built!"
fi

################################################################################
# Step 2: Initialize Wallet with Conway Testnet
################################################################################

log_info "Step 2/6: Initializing wallet with Conway testnet..."

# Create wallet directory
mkdir -p "$HOME/.config/linera-poker"

# Check if wallet already exists
if [ -f "${LINERA_WALLET}" ]; then
    log_warning "Wallet already exists at ${LINERA_WALLET}"
    read -p "Do you want to use existing wallet? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Creating new wallet..."
        rm -f "${LINERA_WALLET}" "${LINERA_KEYSTORE}"
        rm -rf "$HOME/.config/linera-poker/client.db"
        linera wallet init --faucet "${FAUCET_URL}"
        log_success "Wallet initialized!"
    fi
else
    linera wallet init --faucet "${FAUCET_URL}"
    log_success "Wallet initialized!"
fi

################################################################################
# Step 3: Request Chains from Faucet
################################################################################

log_info "Step 3/6: Requesting chains from Conway faucet..."

# Request 3 chains (Table, Player A, Player B)
log_info "Requesting chain 1/3 (Table)..."
TABLE_CHAIN_INFO=($(linera wallet request-chain --faucet "${FAUCET_URL}"))
TABLE_CHAIN_ID="${TABLE_CHAIN_INFO[0]}"
log_success "Table chain created: ${TABLE_CHAIN_ID}"

log_info "Requesting chain 2/3 (Player A)..."
PLAYER_A_CHAIN_INFO=($(linera wallet request-chain --faucet "${FAUCET_URL}"))
PLAYER_A_CHAIN_ID="${PLAYER_A_CHAIN_INFO[0]}"
log_success "Player A chain created: ${PLAYER_A_CHAIN_ID}"

log_info "Requesting chain 3/3 (Player B)..."
PLAYER_B_CHAIN_INFO=($(linera wallet request-chain --faucet "${FAUCET_URL}"))
PLAYER_B_CHAIN_ID="${PLAYER_B_CHAIN_INFO[0]}"
log_success "Player B chain created: ${PLAYER_B_CHAIN_ID}"

################################################################################
# Step 4: Deploy Contracts to Conway
################################################################################

log_info "Step 4/6: Deploying contracts to Conway testnet..."

# Deploy Table Contract (use first chain as default)
log_info "Deploying Table contract..."
TABLE_APP_ID=$(linera publish-and-create \
    "${TABLE_CONTRACT}" "${TABLE_SERVICE}" \
    --json-argument '{"min_stake":100,"max_stake":10000,"small_blind":5,"big_blind":10}' \
    2>&1 | tail -1)
log_success "Table contract deployed: ${TABLE_APP_ID}"

# Deploy Hand Contract for Player A
log_info "Deploying Hand contract for Player A..."
PLAYER_A_HAND_APP_ID=$(linera publish-and-create \
    "${HAND_CONTRACT}" "${HAND_SERVICE}" \
    --json-argument "{\"table_chain\":\"${TABLE_CHAIN_ID}\",\"table_app\":\"${TABLE_APP_ID}\"}" \
    --required-application-ids ${TABLE_APP_ID} \
    2>&1 | tail -1)
log_success "Player A Hand contract deployed: ${PLAYER_A_HAND_APP_ID}"

# Deploy Hand Contract for Player B
log_info "Deploying Hand contract for Player B..."
PLAYER_B_HAND_APP_ID=$(linera publish-and-create \
    "${HAND_CONTRACT}" "${HAND_SERVICE}" \
    --json-argument "{\"table_chain\":\"${TABLE_CHAIN_ID}\",\"table_app\":\"${TABLE_APP_ID}\"}" \
    --required-application-ids ${TABLE_APP_ID} \
    2>&1 | tail -1)
log_success "Player B Hand contract deployed: ${PLAYER_B_HAND_APP_ID}"

################################################################################
# Step 5: Generate Frontend Production .env
################################################################################

log_info "Step 5/6: Generating frontend production configuration..."

cat > frontend/.env.production << EOF
# Linera Poker - Production Environment Configuration
# Conway Testnet Deployment
# Generated: $(date)

# Network Mode
VITE_NETWORK_MODE=conway

# Conway Testnet URLs
VITE_FAUCET_URL=${FAUCET_URL}
VITE_SERVICE_URL=${SERVICE_URL}

# Chain IDs
VITE_TABLE_CHAIN_ID=${TABLE_CHAIN_ID}
VITE_PLAYER_A_CHAIN_ID=${PLAYER_A_CHAIN_ID}
VITE_PLAYER_B_CHAIN_ID=${PLAYER_B_CHAIN_ID}

# Application IDs
VITE_TOKEN_APP_ID=
VITE_TABLE_APP_ID=${TABLE_APP_ID}
VITE_PLAYER_A_HAND_APP_ID=${PLAYER_A_HAND_APP_ID}
VITE_PLAYER_B_HAND_APP_ID=${PLAYER_B_HAND_APP_ID}
EOF

log_success "Production .env created at frontend/.env.production"

################################################################################
# Step 6: Display Deployment Summary
################################################################################

log_info "Step 6/6: Deployment Summary"

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║              🎉  DEPLOYMENT SUCCESSFUL  🎉                     ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "┌────────────────────────────────────────────────────────────────┐"
echo "│                    DEPLOYMENT DETAILS                          │"
echo "├────────────────────────────────────────────────────────────────┤"
echo "│                                                                │"
echo "│  Network:        Conway Testnet                               │"
echo "│  Faucet:         ${FAUCET_URL}  │"
echo "│  Service:        ${SERVICE_URL}    │"
echo "│                                                                │"
echo "├────────────────────────────────────────────────────────────────┤"
echo "│                      CHAIN IDs                                 │"
echo "├────────────────────────────────────────────────────────────────┤"
echo "│                                                                │"
echo "│  Table Chain:    ${TABLE_CHAIN_ID}                            │"
echo "│  Player A Chain: ${PLAYER_A_CHAIN_ID}                         │"
echo "│  Player B Chain: ${PLAYER_B_CHAIN_ID}                         │"
echo "│                                                                │"
echo "├────────────────────────────────────────────────────────────────┤"
echo "│                   APPLICATION IDs                              │"
echo "├────────────────────────────────────────────────────────────────┤"
echo "│                                                                │"
echo "│  Table App:      ${TABLE_APP_ID}                              │"
echo "│  Player A Hand:  ${PLAYER_A_HAND_APP_ID}                      │"
echo "│  Player B Hand:  ${PLAYER_B_HAND_APP_ID}                      │"
echo "│                                                                │"
echo "└────────────────────────────────────────────────────────────────┘"
echo ""

################################################################################
# Next Steps
################################################################################

echo "┌────────────────────────────────────────────────────────────────┐"
echo "│                       NEXT STEPS                               │"
echo "├────────────────────────────────────────────────────────────────┤"
echo "│                                                                │"
echo "│  1. Build Frontend for Production:                            │"
echo "│     cd frontend && npm run build                              │"
echo "│                                                                │"
echo "│  2. Deploy to Netlify:                                        │"
echo "│     npx netlify-cli deploy --prod --dir=frontend/dist        │"
echo "│     (Or drag & drop 'frontend/dist' to Netlify website)      │"
echo "│                                                                │"
echo "│  3. Your poker game will be live at:                         │"
echo "│     https://linera-poker.netlify.app                         │"
echo "│                                                                │"
echo "└────────────────────────────────────────────────────────────────┘"
echo ""

log_success "Deployment complete! Frontend configuration ready for Netlify."
echo ""
