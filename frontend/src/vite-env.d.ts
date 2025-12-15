/// <reference types="vite/client" />

/**
 * TypeScript definitions for Vite environment variables
 *
 * Add your environment variables here to get proper type checking
 * and autocomplete in your IDE.
 */
interface ImportMetaEnv {
  // Dynamic Labs Configuration
  readonly VITE_DYNAMIC_ENVIRONMENT_ID: string

  // Linera Poker Configuration
  readonly VITE_NETWORK_MODE: string
  readonly VITE_SERVICE_URL: string

  // Table Chain (Dealer)
  readonly VITE_TABLE_CHAIN_ID: string
  readonly VITE_TABLE_APP_ID: string

  // Player A Chain
  readonly VITE_PLAYER_A_CHAIN_ID: string
  readonly VITE_PLAYER_A_HAND_APP_ID: string

  // Player B Chain
  readonly VITE_PLAYER_B_CHAIN_ID: string
  readonly VITE_PLAYER_B_HAND_APP_ID: string

  // Game Configuration
  readonly VITE_MIN_STAKE: string
  readonly VITE_MAX_STAKE: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
