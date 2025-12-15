/**
 * Linera Wallet Hook - Dynamic Labs Integration
 *
 * Manages wallet connection to Conway Testnet using Dynamic Labs EVM wallets
 * and bridges them to Linera blockchain via the LineraAdapter.
 *
 * Key Changes from Original:
 * - Uses Dynamic Labs instead of direct Linera wallet
 * - EVM address (MetaMask) bridges to Linera via DynamicSigner
 * - Maintains same hook interface for backward compatibility
 *
 * Connection Flow:
 * 1. User connects EVM wallet via Dynamic Labs UI
 * 2. Hook detects wallet connection
 * 3. LineraAdapter creates Linera wallet with EVM address
 * 4. Returns client + chainId for app usage
 */

import { useState, useEffect, useCallback, useRef } from 'react'
import { useDynamicContext } from '@dynamic-labs/sdk-react-core'
import { lineraAdapter } from '../lib/linera-adapter'
import { blockchainQueryService } from '../services/blockchain-query'
import type { Client } from '@linera/client'

/**
 * Wallet state interface
 */
export interface WalletState {
  client: Client | null
  chainId: string | null
  isConnected: boolean
  isConnecting: boolean
  error: string | null
}

/**
 * Complete return interface for useLineraWallet hook
 */
export interface UseLineraWalletReturn extends WalletState {
  connectWallet: () => Promise<void>
  disconnect: () => void
  requestChain: () => Promise<string | null>
}

/**
 * Conway Testnet faucet URL
 */
const CONWAY_TESTNET_FAUCET = 'https://faucet.testnet-conway.linera.net'

/**
 * Hook to manage Linera wallet connection via Dynamic Labs
 *
 * Automatically connects when Dynamic wallet becomes available
 * Maintains connection state and provides disconnect capability
 *
 * @returns Wallet state and control functions
 */
export function useLineraWallet(): UseLineraWalletReturn {
  // Dynamic Labs context - provides EVM wallet
  const { primaryWallet } = useDynamicContext()

  // Linera connection state
  const [client, setClient] = useState<Client | null>(null)
  const [chainId, setChainId] = useState<string | null>(null)
  const [isConnecting, setIsConnecting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Ref to prevent multiple connection attempts
  const connectionAttemptedRef = useRef(false)

  /**
   * Connects to Linera using the Dynamic Labs wallet
   *
   * This is the core connection function that:
   * 1. Validates Dynamic wallet is available
   * 2. Uses LineraAdapter to connect
   * 3. Updates React state with results
   */
  const connectWallet = useCallback(async () => {
    // Validate Dynamic wallet is available
    if (!primaryWallet) {
      const errorMsg = 'No Dynamic wallet connected. Please connect your wallet first.'
      console.warn('⚠️ [Linera Wallet]', errorMsg)
      setError(errorMsg)
      return
    }

    if (!primaryWallet.address) {
      const errorMsg = 'Dynamic wallet has no address. Please reconnect your wallet.'
      console.warn('⚠️ [Linera Wallet]', errorMsg)
      setError(errorMsg)
      return
    }

    try {
      setIsConnecting(true)
      setError(null)

      console.log('🔵 [Linera Wallet] Initializing Linera connection...')
      console.log('   Dynamic Wallet:', primaryWallet.address)
      console.log('   Faucet:', CONWAY_TESTNET_FAUCET)

      // Use LineraAdapter to handle the complete connection flow
      const provider = await lineraAdapter.connect(
        primaryWallet,
        CONWAY_TESTNET_FAUCET
      )

      console.log('✅ [Linera Wallet] Connection successful!')
      console.log('   Chain ID:', provider.chainId)
      console.log('   Address:', provider.address)

      // Try to initialize blockchain query service
      // Note: This may fail if Client creation timed out, which is expected in browser environments
      try {
        if (provider.client) {
          await blockchainQueryService.initialize(provider.client)
          console.log('✅ [Linera Wallet] Blockchain query service initialized')
        } else {
          console.warn('⚠️ [Linera Wallet] Client not available - blockchain queries will not work')
          console.warn('   This is a known limitation of @linera/client in browser environments')
        }
      } catch (err) {
        console.warn('⚠️ [Linera Wallet] Blockchain query service initialization failed:', err)
      }

      // Update React state - connection is successful even if client failed
      setClient(provider.client)
      setChainId(provider.chainId)
      setIsConnecting(false)
    } catch (err) {
      console.error('❌ [Linera Wallet] Connection failed:', err)

      // Extract user-friendly error message
      let errorMessage = 'Failed to connect to Linera'
      if (err instanceof Error) {
        errorMessage = err.message

        // Make testnet errors more user-friendly
        if (errorMessage.includes('Conway Testnet')) {
          errorMessage = 'Conway Testnet is busy. Please try again in a moment.'
        } else if (errorMessage.includes('claim')) {
          errorMessage = 'Failed to claim chain. The testnet may be congested.'
        }
      }

      setError(errorMessage)
      setIsConnecting(false)
      setClient(null)
      setChainId(null)
    }
  }, [primaryWallet])

  /**
   * Requests an additional chain from the faucet
   *
   * Note: This creates a NEW wallet, not tied to the current connection.
   * May need adjustment based on your use case.
   *
   * @returns Promise resolving to new chainId or null on failure
   */
  const requestChain = useCallback(async (): Promise<string | null> => {
    if (!primaryWallet?.address) {
      console.error('❌ [Linera Wallet] Cannot request chain: no wallet connected')
      return null
    }

    try {
      console.log('🔵 [Linera Wallet] Requesting additional chain...')

      // Get faucet from adapter
      const faucet = lineraAdapter.getFaucet()

      // Create new wallet and claim chain
      const newWallet = await faucet.createWallet()
      const newChainId = await faucet.claimChain(newWallet, primaryWallet.address)

      console.log('✅ [Linera Wallet] New chain created:', newChainId)
      return newChainId
    } catch (err) {
      console.error('❌ [Linera Wallet] Failed to request chain:', err)
      return null
    }
  }, [primaryWallet])

  /**
   * Disconnects the wallet and resets state
   */
  const disconnect = useCallback(() => {
    console.log('🔴 [Linera Wallet] Disconnecting...')

    // Reset blockchain query service
    blockchainQueryService.reset()

    // Reset LineraAdapter state
    lineraAdapter.reset()

    // Reset React state
    setClient(null)
    setChainId(null)
    setIsConnecting(false)
    setError(null)

    // Reset connection attempt flag to allow reconnection
    connectionAttemptedRef.current = false

    console.log('✅ [Linera Wallet] Disconnected successfully')
  }, [])

  /**
   * Auto-connect when Dynamic wallet becomes available
   *
   * This effect watches for:
   * - Dynamic wallet connection
   * - Authentication state changes
   *
   * And automatically initiates Linera connection
   */
  useEffect(() => {
    // Only connect if:
    // 1. Dynamic wallet is connected with address
    // 2. Not already connecting
    // 3. Not already connected (check chainId as source of truth)
    // 4. Haven't attempted connection yet (prevents infinite loop)
    if (
      primaryWallet &&
      primaryWallet.address &&
      !isConnecting &&
      !chainId &&
      !connectionAttemptedRef.current
    ) {
      console.log('🟢 [Linera Wallet] Auto-connecting to Conway Testnet...')
      console.log('   Dynamic Wallet detected:', primaryWallet.address)
      connectionAttemptedRef.current = true
      connectWallet()
    }
  }, [primaryWallet, isConnecting, chainId, connectWallet])

  /**
   * Handle Dynamic wallet disconnection
   *
   * When Dynamic wallet is disconnected, also disconnect Linera
   */
  useEffect(() => {
    if (!primaryWallet && client) {
      console.log('🔴 [Linera Wallet] Dynamic wallet disconnected, cleaning up...')
      disconnect()
    }
  }, [primaryWallet, client, disconnect])

  /**
   * Reset connection attempt when wallet address changes
   *
   * This allows reconnection when user switches to a different wallet
   */
  useEffect(() => {
    if (primaryWallet?.address) {
      connectionAttemptedRef.current = false
    }
  }, [primaryWallet?.address])

  // Return complete wallet state and controls
  return {
    client,
    chainId,
    isConnected: !!chainId,
    isConnecting,
    error,
    connectWallet,
    disconnect,
    requestChain
  }
}
