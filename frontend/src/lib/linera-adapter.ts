/**
 * LineraAdapter - Singleton connection manager for Linera blockchain
 *
 * Handles the complete lifecycle of connecting to Linera using a Dynamic Labs
 * EVM wallet. Implements singleton pattern to prevent multiple WASM initializations
 * and manage connection state safely.
 *
 * Connection Flow:
 * 1. Initialize Linera WASM modules (once)
 * 2. Create faucet connection
 * 3. Create Linera wallet from faucet
 * 4. Claim chain using EVM address
 * 5. Create DynamicSigner bridge
 * 6. Create Linera Client with signer
 *
 * Security considerations:
 * - Singleton prevents race conditions
 * - Connection promise prevents concurrent attempts
 * - Handles WASM "already initialized" gracefully
 * - Validates Dynamic wallet before operations
 *
 * @see Reference: Gmic winner implementation
 */

import initLinera, {
  Faucet,
  Client,
  Wallet,
  Application
} from '@linera/client'
import type { Wallet as DynamicWallet } from '@dynamic-labs/sdk-react-core'
import { DynamicSigner } from './dynamic-signer'

/**
 * Complete Linera provider interface returned after successful connection
 */
export interface LineraProvider {
  /** Linera client for blockchain operations */
  client: Client
  /** Linera wallet instance */
  wallet: Wallet
  /** Faucet instance for testnet operations */
  faucet: Faucet
  /** EVM address used for chain ownership */
  address: string
  /** Claimed chain ID on Linera */
  chainId: string
}

/**
 * LineraAdapter - Manages Linera blockchain connection lifecycle
 *
 * Implements singleton pattern to ensure:
 * - WASM is only initialized once
 * - Connection state is centralized
 * - No race conditions from concurrent connects
 */
export class LineraAdapter {
  /** Singleton instance */
  private static instance: LineraAdapter | null = null

  /** Current connection provider (null if not connected) */
  private provider: LineraProvider | null = null

  /** Current application instance (null if not set) */
  private application: Application | null = null

  /** WASM initialization promise (cached to prevent re-init) */
  private wasmInitPromise: Promise<unknown> | null = null

  /** Active connection promise (prevents concurrent connects) */
  private connectPromise: Promise<LineraProvider> | null = null

  /** Callback fired when connection state changes */
  private onConnectionChange?: () => void

  /** Private constructor (singleton pattern) */
  private constructor() {
    console.log('🔗 [LineraAdapter] Singleton instance created')
  }

  /**
   * Gets the singleton instance of LineraAdapter
   * @returns The singleton LineraAdapter instance
   */
  static getInstance(): LineraAdapter {
    if (!LineraAdapter.instance) {
      LineraAdapter.instance = new LineraAdapter()
    }
    return LineraAdapter.instance
  }

  /**
   * Connects to Linera using a Dynamic Labs EVM wallet
   *
   * This method handles the complete connection flow:
   * 1. Validates Dynamic wallet is available
   * 2. Initializes WASM (if not already done)
   * 3. Creates faucet and wallet
   * 4. Claims chain with EVM address
   * 5. Creates client with DynamicSigner
   *
   * @param dynamicWallet - Connected Dynamic Labs wallet instance
   * @param rpcUrl - Linera faucet RPC URL
   * @returns Promise resolving to LineraProvider with all connection details
   * @throws Error if Dynamic wallet is missing or connection fails
   */
  async connect(
    dynamicWallet: DynamicWallet,
    rpcUrl: string
  ): Promise<LineraProvider> {
    // Return existing provider if already connected
    if (this.provider) {
      console.log('🔗 [LineraAdapter] Already connected, returning existing provider')
      return this.provider
    }

    // Return existing connection attempt if in progress
    if (this.connectPromise) {
      console.log('🔗 [LineraAdapter] Connection in progress, waiting...')
      return this.connectPromise
    }

    // Validate inputs
    if (!dynamicWallet) {
      throw new Error('Dynamic wallet is required for Linera connection')
    }

    if (!dynamicWallet.address) {
      throw new Error('Dynamic wallet must have an address (please connect wallet first)')
    }

    if (!rpcUrl) {
      throw new Error('RPC URL is required for Linera connection')
    }

    try {
      // Create connection promise to prevent concurrent attempts
      this.connectPromise = (async () => {
        const { address } = dynamicWallet
        console.log('🔗 [LineraAdapter] Starting connection with Dynamic wallet:', address)

        // Step 1: Initialize Linera WASM modules
        try {
          if (!this.wasmInitPromise) {
            console.log('🔗 [LineraAdapter] Initializing Linera WASM modules...')
            this.wasmInitPromise = initLinera()
          }
          await this.wasmInitPromise
          console.log('✅ [LineraAdapter] Linera WASM modules initialized successfully')
        } catch (error: unknown) {
          // Handle "already initialized" error gracefully (this is expected on reconnects)
          const errorMessage = error instanceof Error ? error.message : String(error)
          if (errorMessage.includes('storage is already initialized')) {
            console.warn(
              '⚠️ [LineraAdapter] Linera storage already initialized; continuing without re-init'
            )
          } else {
            console.error('❌ [LineraAdapter] WASM initialization failed:', error)
            throw error
          }
        }

        // Step 2: Create faucet instance
        console.log('🔗 [LineraAdapter] Connecting to faucet:', rpcUrl)
        const faucet = await new Faucet(rpcUrl)
        console.log('✅ [LineraAdapter] Faucet connection established')

        // Step 3: Create Linera wallet from faucet
        console.log('🔗 [LineraAdapter] Creating Linera wallet from faucet...')
        const wallet = await faucet.createWallet()
        console.log('✅ [LineraAdapter] Linera wallet created')

        // Step 4: Claim chain using EVM address
        // CRITICAL: This is where we pass the EVM address to Linera
        console.log('🔗 [LineraAdapter] Claiming chain with EVM address:', address)
        const chainId = await faucet.claimChain(wallet, address)
        console.log('✅ [LineraAdapter] Chain claimed successfully!')
        console.log('   Chain ID:', chainId)
        console.log('   Owner:', address)

        // Step 5: Create DynamicSigner to bridge EVM wallet → Linera
        console.log('🔗 [LineraAdapter] Creating DynamicSigner bridge...')
        const signer = new DynamicSigner(dynamicWallet)
        console.log('✅ [LineraAdapter] DynamicSigner created')

        // Step 6: Create Linera Client with DynamicSigner
        // Third parameter is likely a boolean flag (e.g., isLocalWallet)
        console.log('🔗 [LineraAdapter] Creating Linera Client...')
        const client = await new Client(wallet, signer, false)
        console.log('✅ [LineraAdapter] Linera Client created successfully!')

        // Store provider for future use
        this.provider = {
          client,
          wallet,
          faucet,
          chainId,
          address: dynamicWallet.address
        }

        // Notify listeners of connection state change
        this.onConnectionChange?.()

        console.log('🎉 [LineraAdapter] Connection complete!')
        console.log('   EVM Address:', address)
        console.log('   Chain ID:', chainId)

        return this.provider
      })()

      // Wait for connection to complete
      const provider = await this.connectPromise
      return provider
    } catch (error: unknown) {
      console.error('❌ [LineraAdapter] Connection failed:', error)

      // Provide user-friendly error message
      const errorMessage =
        error instanceof Error ? error.message : 'Unknown error occurred'

      throw new Error(`Failed to connect to Linera network: ${errorMessage}`)
    } finally {
      // Clear connection promise to allow retry
      this.connectPromise = null
    }
  }

  /**
   * Sets the active application for operations
   *
   * @param appId - Application ID to set as active
   * @throws Error if not connected or appId is invalid
   */
  async setApplication(appId: string): Promise<void> {
    if (!this.provider) {
      throw new Error('Not connected to Linera - call connect() first')
    }

    if (!appId) {
      throw new Error('Application ID is required')
    }

    console.log('🔗 [LineraAdapter] Setting application:', appId)

    try {
      const application = await this.provider.client.frontend().application(appId)

      if (!application) {
        throw new Error(`Failed to get application with ID: ${appId}`)
      }

      this.application = application
      this.onConnectionChange?.()

      console.log('✅ [LineraAdapter] Application set successfully!')
    } catch (error: unknown) {
      console.error('❌ [LineraAdapter] Failed to set application:', error)
      const errorMessage =
        error instanceof Error ? error.message : 'Unknown error occurred'
      throw new Error(`Failed to set application: ${errorMessage}`)
    }
  }

  /**
   * Queries the current application with a GraphQL-style query
   *
   * @param query - Query object to send to application
   * @returns Promise resolving to query result
   * @throws Error if application not set or query fails
   */
  async queryApplication<T>(query: object): Promise<T> {
    if (!this.application) {
      throw new Error('Application not set - call setApplication() first')
    }

    try {
      console.log('🔗 [LineraAdapter] Querying application...')
      const result = await this.application.query(JSON.stringify(query))
      const response = JSON.parse(result)

      console.log('✅ [LineraAdapter] Application query successful')
      return response as T
    } catch (error: unknown) {
      console.error('❌ [LineraAdapter] Application query failed:', error)
      const errorMessage =
        error instanceof Error ? error.message : 'Unknown error occurred'
      throw new Error(`Failed to query application: ${errorMessage}`)
    }
  }

  /**
   * Gets the current LineraProvider (throws if not connected)
   * @returns Current LineraProvider
   * @throws Error if not connected
   */
  getProvider(): LineraProvider {
    if (!this.provider) {
      throw new Error('Provider not set - not connected to Linera')
    }
    return this.provider
  }

  /**
   * Gets the Faucet instance (throws if not connected)
   * @returns Faucet instance
   * @throws Error if not connected
   */
  getFaucet(): Faucet {
    if (!this.provider?.faucet) {
      throw new Error('Faucet not set - not connected to Linera')
    }
    return this.provider.faucet
  }

  /**
   * Gets the Wallet instance (throws if not connected)
   * @returns Wallet instance
   * @throws Error if not connected
   */
  getWallet(): Wallet {
    if (!this.provider?.wallet) {
      throw new Error('Wallet not set - not connected to Linera')
    }
    return this.provider.wallet
  }

  /**
   * Gets the Application instance (throws if not set)
   * @returns Application instance
   * @throws Error if not set
   */
  getApplication(): Application {
    if (!this.application) {
      throw new Error('Application not set - call setApplication() first')
    }
    return this.application
  }

  /**
   * Checks if a chain is connected
   * @returns true if connected to Linera chain
   */
  isChainConnected(): boolean {
    return this.provider !== null
  }

  /**
   * Checks if an application is set
   * @returns true if application is set
   */
  isApplicationSet(): boolean {
    return this.application !== null
  }

  /**
   * Registers a callback for connection state changes
   * @param callback - Function to call when connection state changes
   */
  onConnectionStateChange(callback: () => void): void {
    this.onConnectionChange = callback
  }

  /**
   * Unregisters the connection state change callback
   */
  offConnectionStateChange(): void {
    this.onConnectionChange = undefined
  }

  /**
   * Resets the adapter state (disconnects)
   * Useful for wallet disconnection or error recovery
   */
  reset(): void {
    console.log('🔴 [LineraAdapter] Resetting connection state...')
    this.application = null
    this.provider = null
    this.connectPromise = null
    // Note: We keep wasmInitPromise to avoid re-initializing WASM
    this.onConnectionChange?.()
    console.log('✅ [LineraAdapter] Connection state reset')
  }
}

/**
 * Export singleton instance for convenient access
 * Usage: import { lineraAdapter } from './linera-adapter'
 */
export const lineraAdapter = LineraAdapter.getInstance()
