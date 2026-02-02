/**
 * LineraAdapter - Singleton connection manager for Linera blockchain
 *
 * Simplified adapter using direct @linera/client (no Dynamic Labs).
 * Connection is now managed by WalletContext; this adapter provides
 * utility methods for application-level operations.
 */

import type { Client, Wallet } from '@linera/client'
import { Faucet } from '@linera/client'

export interface LineraProvider {
  client: Client | null
  wallet: Wallet | null
  faucet: Faucet | null
  address: string
  chainId: string
}

export class LineraAdapter {
  private static instance: LineraAdapter | null = null
  private provider: LineraProvider | null = null

  private constructor() {}

  static getInstance(): LineraAdapter {
    if (!LineraAdapter.instance) {
      LineraAdapter.instance = new LineraAdapter()
    }
    return LineraAdapter.instance
  }

  setProvider(provider: LineraProvider): void {
    this.provider = provider
  }

  getProvider(): LineraProvider | null {
    return this.provider
  }

  getFaucet(): Faucet {
    if (!this.provider?.faucet) {
      throw new Error('Faucet not available - not connected')
    }
    return this.provider.faucet
  }

  isChainConnected(): boolean {
    return this.provider !== null
  }

  reset(): void {
    this.provider = null
  }
}

export const lineraAdapter = LineraAdapter.getInstance()
