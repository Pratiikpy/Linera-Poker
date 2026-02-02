/**
 * Blockchain Query Service - Direct Client Queries
 *
 * Uses @linera/client Application interface to query blockchain state
 * WITHOUT needing external service endpoints.
 *
 * This solves the "service.testnet-conway.linera.net doesn't exist" problem
 * by using the Client's built-in WASM-based query capabilities.
 *
 * Architecture:
 * - Wallet connects to Conway Testnet (already working)
 * - Client is created with wallet + signer
 * - Application instances provide query() method
 * - Queries execute via WASM worker, no HTTP needed
 */

import type { Client, Chain, Application } from '@linera/client'

/**
 * Application cache to avoid recreating Application instances
 */
interface ApplicationCache {
  table?: Application
  playerA?: Application
  playerB?: Application
}

/**
 * Singleton blockchain query service
 */
class BlockchainQueryService {
  private static instance: BlockchainQueryService | null = null
  private applicationCache: ApplicationCache = {}
  private client: Client | null = null
  private chainCache: Map<string, Chain> = new Map()

  private constructor() {
    console.log('[BlockchainQuery] Service initialized')
  }

  static getInstance(): BlockchainQueryService {
    if (!BlockchainQueryService.instance) {
      BlockchainQueryService.instance = new BlockchainQueryService()
    }
    return BlockchainQueryService.instance
  }

  /**
   * Initialize the service with a Linera client
   * Call this after wallet connection succeeds
   */
  async initialize(client: Client): Promise<void> {
    this.client = client
    this.applicationCache = {}
    this.chainCache.clear()
    console.log('[BlockchainQuery] Initialized with client')
  }

  /**
   * Get or create a Chain instance
   */
  private async getChain(chainId: string): Promise<Chain> {
    const cached = this.chainCache.get(chainId)
    if (cached) return cached

    if (!this.client) {
      throw new Error('Client not initialized. Call initialize() first.')
    }

    const chain = await this.client.chain(chainId)
    this.chainCache.set(chainId, chain)
    return chain
  }

  /**
   * Get or create Application instance for a chain/app
   */
  private async getApplication(
    chainId: string,
    appId: string,
    cacheKey: keyof ApplicationCache
  ): Promise<Application> {
    if (this.applicationCache[cacheKey]) {
      return this.applicationCache[cacheKey]!
    }

    if (!this.client) {
      throw new Error('Client not initialized. Call initialize() first.')
    }

    console.log(`[BlockchainQuery] Creating application: ${cacheKey}`)
    console.log(`   Chain: ${chainId.substring(0, 12)}...`)
    console.log(`   App: ${appId.substring(0, 12)}...`)

    try {
      const chain = await this.getChain(chainId)
      const application = await chain.application(appId)

      if (!application) {
        throw new Error(`Failed to create application: ${appId}`)
      }

      this.applicationCache[cacheKey] = application

      console.log(`[BlockchainQuery] Application ${cacheKey} ready`)
      return application
    } catch (error) {
      console.error(`[BlockchainQuery] Failed to create application ${cacheKey}:`, error)
      throw error
    }
  }

  /**
   * Query table state (Dealer chain)
   */
  async queryTableState(
    tableChainId: string,
    tableAppId: string,
    query: object
  ): Promise<any> {
    try {
      console.log('🔍 [BlockchainQuery] Querying table state...')

      const application = await this.getApplication(
        tableChainId,
        tableAppId,
        'table'
      )

      const result = await application.query(JSON.stringify(query))
      const response = JSON.parse(result)

      console.log('✅ [BlockchainQuery] Table state fetched')
      return response
    } catch (error) {
      console.error('❌ [BlockchainQuery] Table query failed:', error)
      throw error
    }
  }

  /**
   * Query player A hand state
   */
  async queryPlayerAState(
    playerAChainId: string,
    playerAAppId: string,
    query: object
  ): Promise<any> {
    try {
      console.log('🔍 [BlockchainQuery] Querying Player A state...')

      const application = await this.getApplication(
        playerAChainId,
        playerAAppId,
        'playerA'
      )

      const result = await application.query(JSON.stringify(query))
      const response = JSON.parse(result)

      console.log('✅ [BlockchainQuery] Player A state fetched')
      return response
    } catch (error) {
      console.error('❌ [BlockchainQuery] Player A query failed:', error)
      throw error
    }
  }

  /**
   * Query player B hand state
   */
  async queryPlayerBState(
    playerBChainId: string,
    playerBAppId: string,
    query: object
  ): Promise<any> {
    try {
      console.log('🔍 [BlockchainQuery] Querying Player B state...')

      const application = await this.getApplication(
        playerBChainId,
        playerBAppId,
        'playerB'
      )

      const result = await application.query(JSON.stringify(query))
      const response = JSON.parse(result)

      console.log('✅ [BlockchainQuery] Player B state fetched')
      return response
    } catch (error) {
      console.error('❌ [BlockchainQuery] Player B query failed:', error)
      throw error
    }
  }

  /**
   * Execute mutation on table (join, bet, reveal)
   */
  async mutateTable(
    tableChainId: string,
    tableAppId: string,
    mutation: object
  ): Promise<any> {
    try {
      console.log('🔍 [BlockchainQuery] Executing table mutation...')

      const application = await this.getApplication(
        tableChainId,
        tableAppId,
        'table'
      )

      // Mutations use the same query() method but with mutation syntax
      const result = await application.query(JSON.stringify(mutation))
      const response = JSON.parse(result)

      console.log('✅ [BlockchainQuery] Table mutation executed')
      return response
    } catch (error) {
      console.error('❌ [BlockchainQuery] Table mutation failed:', error)
      throw error
    }
  }

  /**
   * Reset the service (on disconnect)
   */
  reset(): void {
    console.log('[BlockchainQuery] Resetting service...')
    this.client = null
    this.applicationCache = {}
    this.chainCache.clear()
    console.log('[BlockchainQuery] Service reset')
  }

  /**
   * Check if service is initialized
   */
  isInitialized(): boolean {
    return this.client !== null
  }
}

/**
 * Export singleton instance
 */
export const blockchainQueryService = BlockchainQueryService.getInstance()
