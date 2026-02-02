/**
 * Linera Wallet Hook - Direct @linera/client Integration
 *
 * Provides wallet connection state from the WalletContext.
 * Uses direct @linera/client faucet pattern (no Dynamic Labs).
 *
 * Connection Flow:
 * 1. WalletProvider auto-connects to Conway Testnet faucet
 * 2. Creates wallet, claims chain
 * 3. Returns client + chainId for app usage
 */

import { useWallet } from '../contexts/WalletContext'
import type { Client } from '@linera/client'

export interface WalletState {
  client: Client | null
  chainId: string | null
  isConnected: boolean
  isConnecting: boolean
  error: string | null
}

export interface UseLineraWalletReturn extends WalletState {
  connectWallet: () => Promise<void>
  disconnect: () => void
}

/**
 * Hook to access Linera wallet connection state.
 * The WalletProvider handles auto-connection to Conway Testnet.
 */
export function useLineraWallet(): UseLineraWalletReturn {
  const {
    client,
    chainId,
    isConnected,
    isConnecting,
    error,
    connect,
    disconnect,
  } = useWallet()

  return {
    client,
    chainId,
    isConnected,
    isConnecting,
    error,
    connectWallet: connect,
    disconnect,
  }
}
