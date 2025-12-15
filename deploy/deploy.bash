#!/bin/bash
#
# Linera Poker - Cross-Chain Mental Poker Deployment
#
# ARCHITECTURE:
#   3 contracts on 3 separate chains demonstrating true cross-chain poker
#
#   TABLE_CHAIN (Dealer)
#        |
#        +---> PLAYER_A_CHAIN (Player A's private hand)
#        |
#        +---> PLAYER_B_CHAIN (Player B's private hand)
#
# The key insight: EACH PLAYER'S CARDS ARE ON THEIR OWN CHAIN
# The Table (dealer) CANNOT see player cards - architectural privacy!
#
# DEPLOYMENT ORDER:
#   1. Create 3 chains (table, player_a, player_b)
#   2. Build all WASM contracts
#   3. Deploy Table contract (Dealer)
#   4. Deploy Hand contract for Player A
#   5. Deploy Hand contract for Player B
#   6. Generate frontend/.env
#
# USAGE:
#   ./deploy/deploy.bash               # Deploy to local network (default)
#   ./deploy/deploy.bash local         # Deploy to local network
#   ./deploy/deploy.bash conway        # Deploy to Conway testnet
#   ./deploy/deploy.bash --skip-build  # Skip WASM compilation
#

set -e  # Exit on error
set -o pipefail  # Catch pipe failures

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
WASM_DIR="$PROJECT_ROOT/target/wasm32-unknown-unknown/release"
STATE_FILE="$SCRIPT_DIR/.deploy_state"

# Network mode: local or conway (first positional arg if not a flag)
MODE="local"
if [[ $# -gt 0 && ! "$1" =~ ^-- ]]; then
    MODE="$1"
    shift
fi

# Validate mode
case "$MODE" in
    local|conway)
        # Valid modes
        ;;
    *)
        echo "Invalid mode: $MODE"
        echo "Usage: $0 [local|conway] [OPTIONS]"
        exit 1
        ;;
esac

# Network-specific configuration
FAUCET_LOCAL="http://localhost:8080"
FAUCET_CONWAY="https://faucet.testnet-conway.linera.net"
SERVICE_URL_LOCAL="http://localhost:8080"
SERVICE_URL_CONWAY="https://service.testnet-conway.linera.net"

# Select faucet and service URL based on mode
if [[ "$MODE" == "conway" ]]; then
    FAUCET="$FAUCET_CONWAY"
    SERVICE_URL="$SERVICE_URL_CONWAY"
else
    FAUCET="$FAUCET_LOCAL"
    SERVICE_URL="$SERVICE_URL_LOCAL"
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Parse command line arguments (flags)
SKIP_BUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --clean)
            rm -f "$STATE_FILE"
            echo "Cleaned deployment state"
            exit 0
            ;;
        --help)
            echo "Usage: $0 [MODE] [OPTIONS]"
            echo ""
            echo "Modes:"
            echo "  local          Deploy to local network (default)"
            echo "  conway         Deploy to Conway testnet"
            echo ""
            echo "Options:"
            echo "  --skip-build   Skip WASM compilation step"
            echo "  --clean        Remove deployment state and exit"
            echo "  --help         Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Deploy locally"
            echo "  $0 local              # Deploy locally (explicit)"
            echo "  $0 conway             # Deploy to Conway testnet"
            echo "  $0 conway --skip-build # Deploy to Conway, skip build"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Utility functions - output to stderr so they don't interfere with function return values
print_step() {
    echo -e "${CYAN}[STEP $1]${NC} $2" >&2
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" >&2
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" >&2
}

print_info() {
    echo -e "${MAGENTA}[INFO]${NC} $1" >&2
}

# Save deployment state
save_state() {
    local key="$1"
    local value="$2"
    echo "$key=$value" >> "$STATE_FILE"
}

# Load deployment state
load_state() {
    local key="$1"
    if [[ -f "$STATE_FILE" ]]; then
        grep "^${key}=" "$STATE_FILE" | tail -1 | cut -d'=' -f2
    fi
}

# Validate chain ID format (64-character hex string)
validate_chain_id() {
    local chain_id="$1"
    local name="$2"

    if [[ ! "$chain_id" =~ ^[a-f0-9]{64}$ ]]; then
        print_error "Invalid chain ID for $name: $chain_id"
        print_error "Expected 64-character hex string"
        return 1
    fi
    return 0
}

# Request chain from faucet with retry logic
request_chain() {
    local name="$1"
    local max_retries=3
    local retry_count=0

    while [[ $retry_count -lt $max_retries ]]; do
        print_info "Requesting $name chain from faucet (attempt $((retry_count + 1))/$max_retries)..."

        local output
        if output=$(linera wallet request-chain --faucet "$FAUCET" 2>&1); then
            # Extract chain ID using grep (first 64-char hex string)
            local chain_id=$(echo "$output" | grep -oE '[a-f0-9]{64}' | head -1)

            if validate_chain_id "$chain_id" "$name"; then
                print_success "$name chain: ${chain_id:0:12}..."
                echo "$chain_id"
                return 0
            fi
        fi

        retry_count=$((retry_count + 1))
        if [[ $retry_count -lt $max_retries ]]; then
            print_warning "Retry in 2 seconds..."
            sleep 2
        fi
    done

    print_error "Failed to request $name chain after $max_retries attempts"
    return 1
}

# Deploy contract with proper error handling
deploy_contract() {
    local chain_id="$1"
    local contract_name="$2"
    local instantiation_arg="$3"

    local contract_wasm="$WASM_DIR/${contract_name}_contract.wasm"
    local service_wasm="$WASM_DIR/${contract_name}_service.wasm"

    # Verify WASM files exist
    if [[ ! -f "$contract_wasm" ]]; then
        print_error "Contract WASM not found: $contract_wasm"
        return 1
    fi
    if [[ ! -f "$service_wasm" ]]; then
        print_error "Service WASM not found: $service_wasm"
        return 1
    fi

    print_info "Deploying $contract_name contract to chain ${chain_id:0:12}..."
    print_info "Instantiation argument: $instantiation_arg"

    local output
    local cmd="linera --chain $chain_id publish-and-create $contract_wasm $service_wasm"

    if [[ -n "$instantiation_arg" && "$instantiation_arg" != "null" ]]; then
        cmd="$cmd --json-argument '$instantiation_arg'"
    fi

    if output=$(eval "$cmd" 2>&1); then
        # Extract application ID (last 64-char hex string in output)
        local app_id=$(echo "$output" | grep -oE '[a-f0-9]{64}' | tail -1)

        if validate_chain_id "$app_id" "$contract_name app"; then
            print_success "$contract_name app: ${app_id:0:12}..."
            echo "$app_id"
            return 0
        fi
    fi

    print_error "Failed to deploy $contract_name contract"
    print_error "Output: $output"
    return 1
}

# Start local network (only for local mode)
start_local_network() {
    print_info "Starting local Linera network..."
    if linera net up; then
        print_success "Local network started successfully"
        # Give the network a moment to stabilize
        sleep 2
    else
        print_error "Failed to start local network"
        exit 1
    fi
}

# ============================================================
# MAIN DEPLOYMENT FLOW
# ============================================================

echo "" >&2
echo "============================================================" >&2
echo "   LINERA POKER - Cross-Chain Mental Poker" >&2
echo "   Deployment Script" >&2
echo "============================================================" >&2
echo "" >&2
echo -e "${CYAN}Key Innovation:${NC}" >&2
echo "  Each player's cards are on their OWN chain" >&2
echo "  The dealer CANNOT see them - architectural privacy!" >&2
echo "" >&2
echo -e "${MAGENTA}Network Mode:${NC} $MODE" >&2
echo "Project: $PROJECT_ROOT" >&2
echo "Faucet:  $FAUCET" >&2
echo "" >&2

# ------------------------------------------------------------
# STEP 1: Prerequisites Check
# ------------------------------------------------------------
print_step "1/7" "Checking prerequisites..."

# Check linera CLI
if ! command -v linera >/dev/null 2>&1; then
    print_error "linera CLI not found. Please install Linera SDK first."
    exit 1
fi

LINERA_VERSION=$(linera --version 2>&1 | grep -oP '\d+\.\d+' | head -1 || echo "0.0")
print_info "Linera version: $LINERA_VERSION"

print_success "Prerequisites check passed"

# ------------------------------------------------------------
# STEP 2: Network Setup
# ------------------------------------------------------------
if [[ "$MODE" == "local" ]]; then
    print_step "2/7" "Setting up local network..."
    start_local_network
else
    print_step "2/7" "Using Conway testnet..."
    print_info "Faucet: $FAUCET"
    print_success "Conway testnet selected"
fi

# ------------------------------------------------------------
# STEP 3: Request Chains
# ------------------------------------------------------------
print_step "3/7" "Requesting 3 chains from faucet..."

TABLE_CHAIN=$(load_state "TABLE_CHAIN")
if [[ -z "$TABLE_CHAIN" ]]; then
    TABLE_CHAIN=$(request_chain "Table") || exit 1
    save_state "TABLE_CHAIN" "$TABLE_CHAIN"
else
    print_info "Using existing Table chain: ${TABLE_CHAIN:0:12}..."
fi

PLAYER_A_CHAIN=$(load_state "PLAYER_A_CHAIN")
if [[ -z "$PLAYER_A_CHAIN" ]]; then
    PLAYER_A_CHAIN=$(request_chain "Player_A") || exit 1
    save_state "PLAYER_A_CHAIN" "$PLAYER_A_CHAIN"
else
    print_info "Using existing Player A chain: ${PLAYER_A_CHAIN:0:12}..."
fi

PLAYER_B_CHAIN=$(load_state "PLAYER_B_CHAIN")
if [[ -z "$PLAYER_B_CHAIN" ]]; then
    PLAYER_B_CHAIN=$(request_chain "Player_B") || exit 1
    save_state "PLAYER_B_CHAIN" "$PLAYER_B_CHAIN"
else
    print_info "Using existing Player B chain: ${PLAYER_B_CHAIN:0:12}..."
fi

print_success "All 3 chains requested successfully"

# ------------------------------------------------------------
# STEP 4: Build WASM Contracts
# ------------------------------------------------------------
if [[ "$SKIP_BUILD" == "true" ]]; then
    print_step "4/7" "Skipping WASM build (--skip-build flag)"
else
    print_step "4/7" "Building WASM contracts..."

    cd "$PROJECT_ROOT"
    if cargo build --release --target wasm32-unknown-unknown; then
        print_success "WASM contracts built successfully"
    else
        print_error "Failed to build WASM contracts"
        exit 1
    fi
fi

# ------------------------------------------------------------
# STEP 5: Deploy Table Contract
# ------------------------------------------------------------
print_step "5/7" "Deploying Table contract (Dealer)..."

# Table instantiation: min_stake, max_stake, and blinds
# Standard heads-up poker: small_blind=5, big_blind=10
TABLE_ARG='{"min_stake":10,"max_stake":1000,"small_blind":5,"big_blind":10}'

TABLE_APP=$(load_state "TABLE_APP")
if [[ -z "$TABLE_APP" ]]; then
    TABLE_APP=$(deploy_contract "$TABLE_CHAIN" "table" "$TABLE_ARG") || exit 1
    save_state "TABLE_APP" "$TABLE_APP"
else
    print_info "Using existing Table app: ${TABLE_APP:0:12}..."
fi

# ------------------------------------------------------------
# STEP 6: Deploy Hand Contracts for Players
# ------------------------------------------------------------
print_step "6/7" "Deploying Hand contracts for both players..."

# Hand instantiation: table_chain and table_app
HAND_A_ARG="{\"table_chain\":\"$TABLE_CHAIN\",\"table_app\":\"$TABLE_APP\"}"

HAND_A_APP=$(load_state "HAND_A_APP")
if [[ -z "$HAND_A_APP" ]]; then
    print_info "Deploying Player A's Hand contract..."
    HAND_A_APP=$(deploy_contract "$PLAYER_A_CHAIN" "hand" "$HAND_A_ARG") || exit 1
    save_state "HAND_A_APP" "$HAND_A_APP"
else
    print_info "Using existing Hand A app: ${HAND_A_APP:0:12}..."
fi

HAND_B_ARG="{\"table_chain\":\"$TABLE_CHAIN\",\"table_app\":\"$TABLE_APP\"}"

HAND_B_APP=$(load_state "HAND_B_APP")
if [[ -z "$HAND_B_APP" ]]; then
    print_info "Deploying Player B's Hand contract..."
    HAND_B_APP=$(deploy_contract "$PLAYER_B_CHAIN" "hand" "$HAND_B_ARG") || exit 1
    save_state "HAND_B_APP" "$HAND_B_APP"
else
    print_info "Using existing Hand B app: ${HAND_B_APP:0:12}..."
fi

print_success "Both player Hand contracts deployed"

# ------------------------------------------------------------
# STEP 7: Generate Frontend Configuration
# ------------------------------------------------------------
print_step "7/7" "Generating frontend/.env configuration..."

mkdir -p "$PROJECT_ROOT/frontend"

# Select environment file based on mode
if [[ "$MODE" == "conway" ]]; then
    FRONTEND_ENV="$PROJECT_ROOT/frontend/.env.conway"
else
    FRONTEND_ENV="$PROJECT_ROOT/frontend/.env"
fi

cat > "$FRONTEND_ENV" << EOF
# Linera Poker - Cross-Chain Mental Poker Configuration
# Auto-generated by deploy/deploy.bash
#
# ARCHITECTURE:
#   Table Chain (Dealer) - Manages game, pot, cannot see player cards
#   Player A Chain - Player A's PRIVATE hole cards
#   Player B Chain - Player B's PRIVATE hole cards
#
# KEY INNOVATION:
#   The dealer CANNOT access player chain state
#   Cards are revealed via cross-chain messages only at showdown
#
# Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
# Network Mode: $MODE

# Network Configuration
VITE_NETWORK_MODE=$MODE
VITE_FAUCET_URL=$FAUCET
VITE_SERVICE_URL=$SERVICE_URL

# Table Chain (Dealer)
VITE_TABLE_CHAIN_ID=$TABLE_CHAIN
VITE_TABLE_APP_ID=$TABLE_APP

# Player A Chain
VITE_PLAYER_A_CHAIN_ID=$PLAYER_A_CHAIN
VITE_PLAYER_A_HAND_APP_ID=$HAND_A_APP

# Player B Chain
VITE_PLAYER_B_CHAIN_ID=$PLAYER_B_CHAIN
VITE_PLAYER_B_HAND_APP_ID=$HAND_B_APP

# Game Configuration
VITE_MIN_STAKE=10
VITE_MAX_STAKE=1000
VITE_SMALL_BLIND=5
VITE_BIG_BLIND=10
EOF

print_success "Frontend configuration written to: $FRONTEND_ENV"

# ============================================================
# DEPLOYMENT SUMMARY
# ============================================================

echo ""
echo "============================================================"
echo "   LINERA POKER DEPLOYMENT COMPLETE!"
echo "============================================================"
echo ""
echo -e "${CYAN}Cross-Chain Architecture:${NC}"
echo ""
echo "  ┌─────────────────────────────────────────────────────┐"
echo "  │            TABLE CHAIN (Dealer)                     │"
echo "  │  App: ${TABLE_APP:0:12}...                           │"
echo "  │  • Manages game state machine                       │"
echo "  │  • Holds pot, deals community cards                 │"
echo "  │  • CANNOT see player hole cards!                    │"
echo "  └───────────────────┬─────────────────────────────────┘"
echo "                      │"
echo "          ┌───────────┴───────────┐"
echo "          │                       │"
echo "          ▼                       ▼"
echo "  ┌─────────────────┐     ┌─────────────────┐"
echo "  │  PLAYER A CHAIN │     │  PLAYER B CHAIN │"
echo "  │  ${PLAYER_A_CHAIN:0:12}...  │     │  ${PLAYER_B_CHAIN:0:12}...  │"
echo "  │  PRIVATE cards  │     │  PRIVATE cards  │"
echo "  └─────────────────┘     └─────────────────┘"
echo ""
echo -e "${CYAN}Chain IDs:${NC}"
echo "  Table:    $TABLE_CHAIN"
echo "  Player A: $PLAYER_A_CHAIN"
echo "  Player B: $PLAYER_B_CHAIN"
echo ""
echo -e "${CYAN}Application IDs:${NC}"
echo "  Table App:  $TABLE_APP"
echo "  Hand A App: $HAND_A_APP"
echo "  Hand B App: $HAND_B_APP"
echo ""
echo -e "${GREEN}Ready for demo!${NC}"
echo ""

if [[ "$MODE" == "conway" ]]; then
    echo "Next steps for Conway Testnet:"
    echo "  1. The contracts are now deployed to Conway testnet"
    echo ""
    echo "  2. To use Conway environment in frontend:"
    echo "     cd frontend"
    echo "     cp .env.conway .env"
    echo "     npm install && npm run dev"
    echo ""
    echo "  3. Open two browser windows (one for each player)"
    echo "     http://localhost:5173"
    echo ""
    echo -e "${YELLOW}Note:${NC} Make sure VITE_SERVICE_URL in .env.conway points to a Conway testnet node"
else
    echo "Next steps for Local Development:"
    echo "  1. Start Linera service:"
    echo "     linera service --port 8080"
    echo ""
    echo "  2. Start frontend:"
    echo "     cd frontend && npm install && npm run dev"
    echo ""
    echo "  3. Open two browser windows (one for each player)"
    echo "     http://localhost:5173"
fi

echo ""
echo "============================================================"
echo ""
echo -e "${MAGENTA}THE KEY INSIGHT:${NC}"
echo "  On Ethereum, all cards would be in one contract - dealer can see them."
echo "  On Linera, each player's cards are on their OWN chain."
echo "  The dealer literally CANNOT access them!"
echo ""
echo "============================================================"
