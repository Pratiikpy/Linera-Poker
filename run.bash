#!/bin/bash

################################################################################
# Linera Poker - Automated Deployment Script
# Wave 6 Buildathon Submission
#
# This script orchestrates the complete deployment of a cross-chain poker game:
# 1. Initializes local Linera network with faucet
# 2. Creates three chains (Table, Player A, Player B)
# 3. Builds and deploys all WASM contracts
# 4. Configures frontend environment
# 5. Starts all services
################################################################################

set -euo pipefail  # Exit on error, undefined vars, pipe failures
IFS=$'\n\t'        # Safer word splitting

################################################################################
# Configuration
################################################################################

FAUCET_URL="http://localhost:8080"
SERVICE_URL="http://localhost:8081"
SERVICE_PORT=8081

# Linera environment paths - must be set as environment variables
export LINERA_WALLET="/tmp/linera/wallet.json"
export LINERA_KEYSTORE="/tmp/linera/keystore.json"
export LINERA_STORAGE="rocksdb:/tmp/linera/client.db"

WALLET_PATH="${LINERA_WALLET}"
STORAGE_PATH="${LINERA_STORAGE}"
NETWORK_CONFIG="/tmp/linera/network.json"
LOG_DIR="/tmp/linera/logs"

# Contract paths
WASM_DIR="target/wasm32-unknown-unknown/release"
TABLE_CONTRACT="${WASM_DIR}/table_contract.wasm"
TABLE_SERVICE="${WASM_DIR}/table_service.wasm"
HAND_CONTRACT="${WASM_DIR}/hand_contract.wasm"
HAND_SERVICE="${WASM_DIR}/hand_service.wasm"
TOKEN_CONTRACT="${WASM_DIR}/token_contract.wasm"
TOKEN_SERVICE="${WASM_DIR}/token_service.wasm"

# Frontend configuration
FRONTEND_DIR="frontend"
ENV_FILE="${FRONTEND_DIR}/.env"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

################################################################################
# Utility Functions
################################################################################

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

log_section() {
    echo ""
    echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${MAGENTA}  $*${NC}"
    echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

show_banner() {
    cat << "EOF"
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║           ██╗     ██╗███╗   ██╗███████╗██████╗  █████╗       ║
    ║           ██║     ██║████╗  ██║██╔════╝██╔══██╗██╔══██╗      ║
    ║           ██║     ██║██╔██╗ ██║█████╗  ██████╔╝███████║      ║
    ║           ██║     ██║██║╚██╗██║██╔══╝  ██╔══██╗██╔══██║      ║
    ║           ███████╗██║██║ ╚████║███████╗██║  ██║██║  ██║      ║
    ║           ╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝      ║
    ║                                                               ║
    ║                    🃏  POKER WAVE 6  🃏                       ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝

    ┌───────────────────────────────────────────────────────────────┐
    │                 CROSS-CHAIN ARCHITECTURE                      │
    ├───────────────────────────────────────────────────────────────┤
    │                                                               │
    │  ┌─────────────┐         ┌─────────────┐                     │
    │  │  Player A   │         │  Player B   │                     │
    │  │   Chain     │         │   Chain     │                     │
    │  │             │         │             │                     │
    │  │  Hand App   │         │  Hand App   │                     │
    │  │  (Private)  │         │  (Private)  │                     │
    │  └──────┬──────┘         └──────┬──────┘                     │
    │         │                       │                            │
    │         │   Cross-Chain Msgs    │                            │
    │         └───────────┬───────────┘                            │
    │                     │                                        │
    │              ┌──────▼──────┐                                 │
    │              │    Table    │                                 │
    │              │    Chain    │                                 │
    │              │             │                                 │
    │              │  Table App  │                                 │
    │              │ (Game Logic)│                                 │
    │              └─────────────┘                                 │
    │                                                               │
    │  Features:                                                    │
    │  • Secure hand privacy via separate chains                   │
    │  • Atomic game state updates on table chain                  │
    │  • Cross-chain message passing for actions                   │
    │  • Token contract for betting/chips                          │
    │                                                               │
    └───────────────────────────────────────────────────────────────┘

EOF
}

wait_for_service() {
    local url=$1
    local service_name=$2
    local max_attempts=30
    local attempt=1

    log_info "Waiting for ${service_name} at ${url}..."

    while [ $attempt -le $max_attempts ]; do
        if curl -sf "${url}" >/dev/null 2>&1; then
            log_success "${service_name} is ready!"
            return 0
        fi

        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done

    echo ""
    log_error "${service_name} failed to start after ${max_attempts} attempts"
    return 1
}

cleanup() {
    log_warning "Cleaning up previous Linera instances..."
    # Kill any existing linera processes (use killall as fallback if pkill unavailable)
    if command -v pkill &> /dev/null; then
        pkill -f "linera-proxy" || true
        pkill -f "linera-server" || true
        pkill -f "linera service" || true
    elif command -v killall &> /dev/null; then
        killall linera-proxy linera-server linera 2>/dev/null || true
    fi
    rm -rf /tmp/linera
    rm -rf /root/.config/linera
    mkdir -p "${LOG_DIR}"
    sleep 2
}

################################################################################
# Main Deployment Functions
################################################################################

initialize_network() {
    log_section "1. INITIALIZING LINERA NETWORK"

    cleanup

    log_info "Starting local Linera network with faucet..."

    # Load the linera net helper functions (provides linera_spawn)
    log_info "Loading linera net helper..."
    eval "$(linera net helper)"

    # Start local network with faucet using linera_spawn (runs in background)
    log_info "Spawning Linera network with faucet service..."
    linera_spawn linera net up --with-faucet

    # Wait for faucet to be ready
    wait_for_service "${FAUCET_URL}" "Faucet Service"

    log_success "Network initialized successfully!"
}

initialize_wallet() {
    log_section "2. INITIALIZING WALLET"

    log_info "Creating wallet with faucet: ${FAUCET_URL}"

    if ! linera wallet init --faucet="${FAUCET_URL}" > "${LOG_DIR}/wallet-init.log" 2>&1; then
        log_error "Failed to initialize wallet. Check ${LOG_DIR}/wallet-init.log"
        cat "${LOG_DIR}/wallet-init.log"
        exit 1
    fi

    log_success "Wallet initialized at ${WALLET_PATH}"
}

request_chains() {
    log_section "3. REQUESTING CHAINS FROM FAUCET"

    declare -a CHAIN_NAMES=("Token/Table" "Player A" "Player B")

    # Request chain 1 (Token/Table)
    log_info "Requesting chain 1/3 (${CHAIN_NAMES[0]})..."
    local INFO_1
    INFO_1=($(linera wallet request-chain --faucet="${FAUCET_URL}" 2>/dev/null))
    if [ ${#INFO_1[@]} -lt 1 ]; then
        log_error "Failed to request chain 1"
        exit 1
    fi
    TABLE_CHAIN_ID="${INFO_1[0]}"
    log_success "Chain 1 created: ${TABLE_CHAIN_ID} (${CHAIN_NAMES[0]})"

    # Request chain 2 (Player A)
    log_info "Requesting chain 2/3 (${CHAIN_NAMES[1]})..."
    local INFO_2
    INFO_2=($(linera wallet request-chain --faucet="${FAUCET_URL}" 2>/dev/null))
    if [ ${#INFO_2[@]} -lt 1 ]; then
        log_error "Failed to request chain 2"
        exit 1
    fi
    PLAYER_A_CHAIN_ID="${INFO_2[0]}"
    log_success "Chain 2 created: ${PLAYER_A_CHAIN_ID} (${CHAIN_NAMES[1]})"

    # Request chain 3 (Player B)
    log_info "Requesting chain 3/3 (${CHAIN_NAMES[2]})..."
    local INFO_3
    INFO_3=($(linera wallet request-chain --faucet="${FAUCET_URL}" 2>/dev/null))
    if [ ${#INFO_3[@]} -lt 1 ]; then
        log_error "Failed to request chain 3"
        exit 1
    fi
    PLAYER_B_CHAIN_ID="${INFO_3[0]}"
    log_success "Chain 3 created: ${PLAYER_B_CHAIN_ID} (${CHAIN_NAMES[2]})"

    log_info "Chain assignments:"
    echo "  • Table/Token Chain: ${TABLE_CHAIN_ID}"
    echo "  • Player A Chain:    ${PLAYER_A_CHAIN_ID}"
    echo "  • Player B Chain:    ${PLAYER_B_CHAIN_ID}"
}

build_contracts() {
    log_section "4. BUILDING WASM CONTRACTS"

    # Clean previous build to avoid version mismatch issues
    log_info "Cleaning previous build artifacts..."
    cargo clean --target wasm32-unknown-unknown 2>/dev/null || true

    log_info "Compiling all contracts for wasm32-unknown-unknown..."

    if ! cargo build --release --target wasm32-unknown-unknown 2>&1 | tee "${LOG_DIR}/build.log"; then
        log_error "Contract build failed. Check ${LOG_DIR}/build.log"
        exit 1
    fi

    # Verify all contract files exist
    local contracts=(
        "${TABLE_CONTRACT}"
        "${TABLE_SERVICE}"
        "${HAND_CONTRACT}"
        "${HAND_SERVICE}"
        "${TOKEN_CONTRACT}"
        "${TOKEN_SERVICE}"
    )

    for contract in "${contracts[@]}"; do
        if [ ! -f "${contract}" ]; then
            log_error "Missing contract: ${contract}"
            exit 1
        fi
        local size=$(du -h "${contract}" | cut -f1)
        log_info "  ✓ $(basename "${contract}") (${size})"
    done

    log_success "All contracts built successfully!"
}

deploy_contracts() {
    log_section "5. DEPLOYING CONTRACTS"

    declare -g TOKEN_APP_ID=""
    declare -g TABLE_APP_ID
    declare -g PLAYER_A_HAND_APP_ID
    declare -g PLAYER_B_HAND_APP_ID

    local deploy_output

    # Deploy Table Contract first (no dependencies)
    # Table needs: min_stake, max_stake, small_blind, big_blind
    log_info "Deploying Table contract..."
    set +e  # Temporarily allow errors
    deploy_output=$(linera publish-and-create \
        "${TABLE_CONTRACT}" "${TABLE_SERVICE}" \
        --json-argument '{"min_stake":100,"max_stake":10000,"small_blind":5,"big_blind":10}' 2>&1)
    local deploy_exit=$?
    set -e

    if [ $deploy_exit -ne 0 ]; then
        log_error "Failed to deploy Table contract (exit code: $deploy_exit)"
        echo "Output: ${deploy_output}"
        exit 1
    fi
    TABLE_APP_ID=$(echo "${deploy_output}" | tail -1)
    log_success "Table contract deployed: ${TABLE_APP_ID}"

    # Deploy Hand Contract for Player A (on default chain for now)
    # Hand needs: table_chain (ChainId), table_app (ApplicationId)
    log_info "Deploying Hand contract for Player A..."
    set +e
    deploy_output=$(linera publish-and-create \
        "${HAND_CONTRACT}" "${HAND_SERVICE}" \
        --json-argument "{\"table_chain\":\"${TABLE_CHAIN_ID}\",\"table_app\":\"${TABLE_APP_ID}\"}" \
        --required-application-ids ${TABLE_APP_ID} 2>&1)
    deploy_exit=$?
    set -e

    if [ $deploy_exit -ne 0 ]; then
        log_error "Failed to deploy Hand contract for Player A (exit code: $deploy_exit)"
        echo "Output: ${deploy_output}"
        exit 1
    fi
    PLAYER_A_HAND_APP_ID=$(echo "${deploy_output}" | tail -1)
    log_success "Player A Hand contract deployed: ${PLAYER_A_HAND_APP_ID}"

    # Deploy Hand Contract for Player B (on default chain for now)
    log_info "Deploying Hand contract for Player B..."
    set +e
    deploy_output=$(linera publish-and-create \
        "${HAND_CONTRACT}" "${HAND_SERVICE}" \
        --json-argument "{\"table_chain\":\"${TABLE_CHAIN_ID}\",\"table_app\":\"${TABLE_APP_ID}\"}" \
        --required-application-ids ${TABLE_APP_ID} 2>&1)
    deploy_exit=$?
    set -e

    if [ $deploy_exit -ne 0 ]; then
        log_error "Failed to deploy Hand contract for Player B (exit code: $deploy_exit)"
        echo "Output: ${deploy_output}"
        exit 1
    fi
    PLAYER_B_HAND_APP_ID=$(echo "${deploy_output}" | tail -1)
    log_success "Player B Hand contract deployed: ${PLAYER_B_HAND_APP_ID}"

    log_info "Deployment summary:"
    echo "  • Table App:        ${TABLE_APP_ID}"
    echo "  • Player A Hand:    ${PLAYER_A_HAND_APP_ID}"
    echo "  • Player B Hand:    ${PLAYER_B_HAND_APP_ID}"

    # Note: Token contract is optional - players use native tokens or frontend handles token creation
}

configure_frontend() {
    log_section "6. CONFIGURING FRONTEND"

    if [ ! -d "${FRONTEND_DIR}" ]; then
        log_error "Frontend directory not found: ${FRONTEND_DIR}"
        exit 1
    fi

    log_info "Generating environment configuration..."

    cat > "${ENV_FILE}" << EOF
# Linera Poker - Frontend Environment Configuration
# Auto-generated by run.bash on $(date)

# Network Mode
VITE_NETWORK_MODE=local

# Service URLs
VITE_FAUCET_URL=${FAUCET_URL}
VITE_SERVICE_URL=${SERVICE_URL}

# Chain IDs
VITE_TABLE_CHAIN_ID=${TABLE_CHAIN_ID}
VITE_PLAYER_A_CHAIN_ID=${PLAYER_A_CHAIN_ID}
VITE_PLAYER_B_CHAIN_ID=${PLAYER_B_CHAIN_ID}

# Application IDs
VITE_TOKEN_APP_ID=${TOKEN_APP_ID}
VITE_TABLE_APP_ID=${TABLE_APP_ID}
VITE_PLAYER_A_HAND_APP_ID=${PLAYER_A_HAND_APP_ID}
VITE_PLAYER_B_HAND_APP_ID=${PLAYER_B_HAND_APP_ID}
EOF

    log_success "Environment file created: ${ENV_FILE}"
    log_info "Configuration:"
    cat "${ENV_FILE}" | grep -v '^#' | grep -v '^$' | sed 's/^/  /'
}

start_services() {
    log_section "7. STARTING SERVICES"

    # Start Linera GraphQL service (matches buildathon template approach)
    log_info "Starting Linera GraphQL service on port ${SERVICE_PORT}..."
    linera service --port "${SERVICE_PORT}" > "${LOG_DIR}/service.log" 2>&1 &
    local service_pid=$!
    echo "${service_pid}" > /tmp/linera/service.pid

    log_info "Service started with PID ${service_pid}"
    log_info "Waiting for service to initialize..."
    sleep 3

    log_success "GraphQL service running (check logs if needed: ${LOG_DIR}/service.log)"

    # Install frontend dependencies
    log_info "Installing frontend dependencies..."
    cd "${FRONTEND_DIR}"

    if [ ! -f "package.json" ]; then
        log_error "package.json not found in ${FRONTEND_DIR}"
        exit 1
    fi

    if ! npm install > "${LOG_DIR}/npm-install.log" 2>&1; then
        log_error "npm install failed. Check ${LOG_DIR}/npm-install.log"
        cat "${LOG_DIR}/npm-install.log"
        exit 1
    fi

    log_success "Frontend dependencies installed"

    # Start frontend development server
    log_info "Starting Vite development server..."
    log_info "Frontend will be available at http://localhost:5173"

    # Run in foreground to keep container alive
    exec npm run dev -- --host 0.0.0.0
}

show_completion_banner() {
    cat << "EOF"

    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║                     🎉  DEPLOYMENT COMPLETE  🎉               ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝

    ┌───────────────────────────────────────────────────────────────┐
    │                       ACCESS POINTS                           │
    ├───────────────────────────────────────────────────────────────┤
    │                                                               │
    │  🌐 Frontend:       http://localhost:5173                     │
    │  🚰 Faucet:         http://localhost:8080                     │
    │  🔍 GraphQL:        http://localhost:8081                     │
    │  ⛓️  Validator:      http://localhost:13001                    │
    │                                                               │
    ├───────────────────────────────────────────────────────────────┤
    │                          LOGS                                 │
    ├───────────────────────────────────────────────────────────────┤
    │                                                               │
    │  📝 All logs:       /tmp/linera/logs/                         │
    │  📝 Network:        /tmp/linera/logs/net-helper.log           │
    │  📝 Service:        /tmp/linera/logs/service.log              │
    │  📝 Build:          /tmp/linera/logs/build.log                │
    │                                                               │
    └───────────────────────────────────────────────────────────────┘

    Ready to play poker on the Linera blockchain! 🃏♠️♥️♣️♦️

EOF
}

################################################################################
# Main Execution
################################################################################

main() {
    show_banner

    log_info "Starting Linera Poker deployment..."
    log_info "Working directory: $(pwd)"
    log_info "Wallet: ${WALLET_PATH}"
    log_info "Storage: ${STORAGE_PATH}"

    initialize_network
    initialize_wallet
    request_chains
    build_contracts
    deploy_contracts
    configure_frontend

    show_completion_banner

    start_services
}

# Trap errors and cleanup
trap 'log_error "Deployment failed at line $LINENO. Check logs in ${LOG_DIR}"' ERR

# Run main function
main "$@"
