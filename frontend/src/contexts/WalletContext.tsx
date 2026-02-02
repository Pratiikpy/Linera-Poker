import { createContext, useContext, ReactNode, useState, useCallback, useRef, useEffect } from 'react'
import { initialize, Faucet, Client, Chain, signer } from '@linera/client'

// Faucet URL - use env var or default to Conway Testnet
const FAUCET_URL = import.meta.env.VITE_FAUCET_URL || 'https://faucet.testnet-conway.linera.net'

export interface WalletAccount {
  address: string
}

interface WalletContextType {
  primaryWallet: WalletAccount | null
  chainId: string | null
  client: Client | null
  chain: Chain | null
  isConnected: boolean
  isConnecting: boolean
  error: string | null
  connect: () => Promise<void>
  disconnect: () => void
}

const WalletContext = createContext<WalletContextType | undefined>(undefined)

export const useWallet = () => {
  const context = useContext(WalletContext)
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider')
  }
  return context
}

export const WalletProvider = ({ children }: { children: ReactNode }) => {
  const [wallet, setWallet] = useState<WalletAccount | null>(null)
  const [chainId, setChainId] = useState<string | null>(null)
  const [client, setClient] = useState<Client | null>(null)
  const [chain, setChain] = useState<Chain | null>(null)
  const [isConnecting, setIsConnecting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const wasmInitRef = useRef<Promise<unknown> | null>(null)
  const autoConnectAttempted = useRef(false)

  const connect = useCallback(async () => {
    if (isConnecting) return

    try {
      setIsConnecting(true)
      setError(null)

      console.log('[Linera Wallet] Connecting to faucet:', FAUCET_URL)

      // Step 1: Initialize WASM (once)
      if (!wasmInitRef.current) {
        console.log('[Linera Wallet] Initializing WASM...')
        wasmInitRef.current = initialize()
      }

      try {
        await wasmInitRef.current
        console.log('[Linera Wallet] WASM initialized')
      } catch (err: unknown) {
        const msg = err instanceof Error ? err.message : String(err)
        if (!msg.includes('already initialized')) {
          throw err
        }
      }

      // Step 2: Create signer (in-memory for demo)
      console.log('[Linera Wallet] Creating signer...')
      const privateKey = signer.PrivateKey.createRandom()
      const ownerAddress = privateKey.address()

      // Step 3: Create faucet connection
      console.log('[Linera Wallet] Creating faucet connection...')
      const faucet = new Faucet(FAUCET_URL)

      // Step 4: Create wallet
      console.log('[Linera Wallet] Creating wallet...')
      const lineraWallet = await faucet.createWallet()

      // Step 5: Claim chain (needs wallet + owner)
      console.log('[Linera Wallet] Claiming chain...')
      const claimedChainId = await faucet.claimChain(lineraWallet, ownerAddress)
      console.log('[Linera Wallet] Chain claimed:', claimedChainId)

      // Step 6: Create client with wallet + signer
      let lineraClient: Client | null = null
      let lineraChain: Chain | null = null
      try {
        lineraClient = await Promise.race([
          new Client(lineraWallet, privateKey),
          new Promise<Client>((_, reject) =>
            setTimeout(() => reject(new Error('Client creation timeout')), 15000)
          )
        ]) as Client
        console.log('[Linera Wallet] Client created')

        // Step 7: Connect to chain
        lineraChain = await lineraClient.chain(claimedChainId)
        console.log('[Linera Wallet] Chain connected')
      } catch (clientErr) {
        console.warn('[Linera Wallet] Client creation failed, proceeding with basic connection:', clientErr)
      }

      // Generate a deterministic address from chain ID for display
      const displayAddress = `0x${claimedChainId.substring(0, 40)}`

      setWallet({ address: displayAddress })
      setChainId(claimedChainId)
      setClient(lineraClient)
      setChain(lineraChain)
      setIsConnecting(false)

      console.log('[Linera Wallet] Connected successfully!')
      console.log('  Chain ID:', claimedChainId)
    } catch (err) {
      console.error('[Linera Wallet] Connection failed:', err)
      const errorMessage = err instanceof Error ? err.message : 'Failed to connect'
      setError(errorMessage)
      setIsConnecting(false)
    }
  }, [isConnecting])

  const disconnect = useCallback(() => {
    console.log('[Linera Wallet] Disconnecting...')
    setWallet(null)
    setChainId(null)
    setClient(null)
    setChain(null)
    setError(null)
    autoConnectAttempted.current = false
  }, [])

  // Auto-connect on mount
  useEffect(() => {
    if (!autoConnectAttempted.current && !wallet && !isConnecting) {
      autoConnectAttempted.current = true
      console.log('[Linera Wallet] Auto-connecting to Conway Testnet...')
      connect()
    }
  }, [connect, wallet, isConnecting])

  const value: WalletContextType = {
    primaryWallet: wallet,
    chainId,
    client,
    chain,
    isConnected: !!chainId,
    isConnecting,
    error,
    connect,
    disconnect,
  }

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  )
}

// Simple wrapper for connect button
export const ConnectWalletWrapper = ({ children }: { children: ReactNode }) => {
  const { connect, isConnected, disconnect } = useWallet()

  return (
    <div onClick={isConnected ? disconnect : () => connect()}>
      {children}
    </div>
  )
}
