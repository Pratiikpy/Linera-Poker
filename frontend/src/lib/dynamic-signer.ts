/**
 * DynamicSigner - Bridges EVM wallet signatures to Linera signing interface
 *
 * CRITICAL: This implementation uses `personal_sign` directly instead of `signMessage`
 * to avoid double-hashing. The Linera protocol pre-hashes messages before signing,
 * and standard wallet signing methods would hash again, causing signature verification
 * to fail.
 *
 * Security considerations:
 * - Validates owner address matches connected wallet
 * - Enforces lowercase address normalization
 * - Checks for empty/invalid signatures
 *
 * @see Reference: Gmic winner implementation
 */

import type { Signer } from '@linera/client'
import type { Wallet as DynamicWallet } from '@dynamic-labs/sdk-react-core'
import { isEthereumWallet } from '@dynamic-labs/ethereum'

/**
 * Implements Linera's Signer interface using a Dynamic Labs EVM wallet
 */
export class DynamicSigner implements Signer {
  private dynamicWallet: DynamicWallet

  constructor(dynamicWallet: DynamicWallet) {
    if (!dynamicWallet) {
      throw new Error('DynamicSigner requires a valid Dynamic wallet instance')
    }

    if (!dynamicWallet.address) {
      throw new Error('Dynamic wallet must have an address')
    }

    this.dynamicWallet = dynamicWallet

    console.log('🔐 [DynamicSigner] Initialized with wallet:', dynamicWallet.address)
  }

  /**
   * Returns the wallet's EVM address
   * @returns Promise resolving to the wallet address (0x...)
   */
  async address(): Promise<string> {
    const addr = this.dynamicWallet.address
    if (!addr) {
      throw new Error('Wallet address is unavailable')
    }
    return addr
  }

  /**
   * Checks if this signer contains the given owner address
   * Performs case-insensitive comparison
   *
   * @param owner - The owner address to check
   * @returns Promise resolving to true if owner matches wallet address
   */
  async containsKey(owner: string): Promise<boolean> {
    if (!owner) {
      return false
    }

    const walletAddress = this.dynamicWallet.address
    if (!walletAddress) {
      return false
    }

    // Normalize both addresses to lowercase for comparison
    const normalizedOwner = owner.toLowerCase()
    const normalizedWallet = walletAddress.toLowerCase()

    return normalizedOwner === normalizedWallet
  }

  /**
   * Signs a message using the EVM wallet
   *
   * CRITICAL IMPLEMENTATION NOTE:
   * This method uses `personal_sign` directly via the wallet client, NOT the
   * standard `signMessage` method. This is because:
   *
   * 1. The `value` parameter is already pre-hashed by Linera
   * 2. Using `signMessage` would hash it again (double-hash)
   * 3. This would cause signature verification to fail
   *
   * DO NOT CHANGE THIS TO USE signMessage() WITHOUT UNDERSTANDING THE IMPLICATIONS!
   *
   * @param owner - The owner address that should sign (must match wallet)
   * @param value - Pre-hashed message bytes to sign
   * @returns Promise resolving to the signature hex string
   * @throws Error if owner doesn't match wallet or signing fails
   */
  async sign(owner: string, value: Uint8Array): Promise<string> {
    // Validate inputs
    if (!owner) {
      throw new Error('Owner address is required for signing')
    }

    if (!value || value.length === 0) {
      throw new Error('Message value cannot be empty')
    }

    const primaryWallet = this.dynamicWallet.address

    if (!primaryWallet) {
      throw new Error('No wallet address found - wallet may be disconnected')
    }

    // Security check: Ensure the owner matches the connected wallet
    const normalizedOwner = owner.toLowerCase()
    const normalizedWallet = primaryWallet.toLowerCase()

    if (normalizedOwner !== normalizedWallet) {
      throw new Error(
        `Owner mismatch: requested ${owner} but wallet is ${primaryWallet}`
      )
    }

    try {
      // Convert Uint8Array to hex string for signing
      const msgHex: `0x${string}` = `0x${uint8ArrayToHex(value)}`
      const address: `0x${string}` = owner as `0x${string}`

      console.log('🔐 [DynamicSigner] Signing message:', {
        owner: `${owner.substring(0, 10)}...`,
        messageLength: value.length,
        messageHex: `${msgHex.substring(0, 20)}...`
      })

      // CRITICAL: Must check if wallet is Ethereum-compatible
      if (!isEthereumWallet(this.dynamicWallet)) {
        throw new Error('Wallet is not an Ethereum-compatible wallet')
      }

      // Get the wallet client for low-level RPC access
      const walletClient = await this.dynamicWallet.getWalletClient()

      if (!walletClient) {
        throw new Error('Failed to get wallet client from Dynamic wallet')
      }

      // IMPORTANT: Use personal_sign directly to avoid double-hashing
      // DO NOT use walletClient.signMessage() or this.dynamicWallet.signMessage()
      const signature = await walletClient.request({
        method: 'personal_sign',
        params: [msgHex, address]
      })

      if (!signature) {
        throw new Error('Signature request returned empty result')
      }

      // Validate signature format (should start with 0x and be 132 chars: 0x + 130 hex)
      if (typeof signature !== 'string' || !signature.startsWith('0x')) {
        throw new Error('Invalid signature format received from wallet')
      }

      console.log('✅ [DynamicSigner] Message signed successfully:', {
        signatureLength: signature.length,
        signaturePrefix: signature.substring(0, 10)
      })

      return signature
    } catch (error: unknown) {
      console.error('❌ [DynamicSigner] Signing failed:', error)

      // Provide user-friendly error messages
      if (error instanceof Error) {
        if (error.message.includes('User rejected')) {
          throw new Error('Signature request was rejected by user')
        }
        if (error.message.includes('User denied')) {
          throw new Error('Signature request was denied by user')
        }
        throw new Error(`Failed to sign message: ${error.message}`)
      }

      throw new Error('Failed to sign message: Unknown error occurred')
    }
  }
}

/**
 * Converts a Uint8Array to a hex string (without 0x prefix)
 *
 * @param bytes - The byte array to convert
 * @returns Hex string representation
 */
function uint8ArrayToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte: number) => byte.toString(16).padStart(2, '0'))
    .join('')
}
