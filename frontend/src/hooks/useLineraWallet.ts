/**
 * Linera Wallet Hook
 *
 * Manages wallet connection to Conway Testnet using @linera/client
 * This is REQUIRED for buildathon judging - must connect on page load
 */

import { useState, useEffect, useCallback } from 'react'

// Types for Linera client (these match the @linera/client API)
type LineraClient = any // The actual Client from @linera/client
type LineraFaucet = any // The actual Faucet from @linera/client

export interface WalletState {
  client: LineraClient | null
  chainId: string | null
  isConnected: boolean
  isConnecting: boolean
  error: string | null
}

export interface UseLineraWalletReturn extends WalletState {
  connectWallet: () => Promise<void>
  disconnect: () => void
  requestChain: () => Promise<string | null>
}

/**
 * Hook to manage Linera wallet connection
 * Automatically connects to Conway Testnet on mount
 */
export function useLineraWallet(): UseLineraWalletReturn {
  const [client, setClient] = useState<LineraClient | null>(null)
  const [chainId, setChainId] = useState<string | null>(null)
  const [isConnecting, setIsConnecting] = useState(true) // Start as connecting
  const [error, setError] = useState<string | null>(null)

  /**
   * Initialize wallet connection to Conway Testnet
   * This is called automatically on mount
   */
  const connectWallet = useCallback(async () => {
    try {
      setIsConnecting(true)
      setError(null)

      console.log('🔵 [Linera Wallet] Initializing Linera client...')

      // Dynamically import @linera/client to avoid SSR issues
      const linera = await import('@linera/client')

      console.log('🔵 [Linera Wallet] Initializing WASM...')
      // Initialize Linera WASM module
      await linera.default()

      console.log('🔵 [Linera Wallet] Connecting to Conway Testnet faucet...')
      // Connect to Conway Testnet faucet
      const faucet: LineraFaucet = new linera.Faucet(
        'https://faucet.testnet-conway.linera.net'
      )

      console.log('🔵 [Linera Wallet] Creating wallet from faucet...')
      // Create wallet from faucet
      const wallet = await faucet.createWallet()

      console.log('🔵 [Linera Wallet] Creating client...')
      // Create client with wallet storage, signer, and RPC endpoint
      // Based on Linera client library signature
      const newClient: LineraClient = new linera.Client(
        wallet.storage,
        wallet.signer,
        'https://rpc.testnet-conway.linera.net:8080'
      )

      console.log('🔵 [Linera Wallet] Requesting chain with tokens...')
      // Request a chain with tokens from faucet
      const newChainId: string = await faucet.claimChain(newClient)

      console.log('✅ [Linera Wallet] Successfully connected!')
      console.log(`   Chain ID: ${newChainId}`)

      setClient(newClient)
      setChainId(newChainId)
      setIsConnecting(false)

    } catch (err) {
      console.error('❌ [Linera Wallet] Connection failed:', err)
      const errorMessage = err instanceof Error ? err.message : 'Unknown wallet error'
      setError(errorMessage)
      setIsConnecting(false)
    }
  }, [])

  /**
   * Request an additional chain from the faucet
   */
  const requestChain = useCallback(async (): Promise<string | null> => {
    if (!client) {
      console.error('❌ [Linera Wallet] Cannot request chain: no client')
      return null
    }

    try {
      console.log('🔵 [Linera Wallet] Requesting additional chain...')

      const linera = await import('@linera/client')
      const faucet: LineraFaucet = new linera.Faucet(
        'https://faucet.testnet-conway.linera.net'
      )

      // Use existing client to claim chain
      const newChainId: string = await faucet.claimChain(client)

      console.log('✅ [Linera Wallet] New chain created:', newChainId)
      return newChainId

    } catch (err) {
      console.error('❌ [Linera Wallet] Failed to request chain:', err)
      return null
    }
  }, [client])

  /**
   * Disconnect wallet and reset state
   */
  const disconnect = useCallback(() => {
    console.log('🔴 [Linera Wallet] Disconnecting...')
    setClient(null)
    setChainId(null)
    setIsConnecting(false)
    setError(null)
  }, [])

  /**
   * Auto-connect on mount (REQUIRED for buildathon)
   */
  useEffect(() => {
    console.log('🟢 [Linera Wallet] Auto-connecting to Conway Testnet...')
    connectWallet()
  }, [connectWallet])

  return {
    client,
    chainId,
    isConnected: !!client && !!chainId,
    isConnecting,
    error,
    connectWallet,
    disconnect,
    requestChain,
  }
}
