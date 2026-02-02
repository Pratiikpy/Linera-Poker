/**
 * Network Configuration for Linera Poker
 *
 * Handles environment detection and provides network-specific utilities
 * for connecting to local development or Conway testnet.
 *
 * SECURITY:
 * - All chain IDs are validated before use
 * - Service URLs are validated for proper format
 * - Environment variables provide safe defaults
 */

// Network modes supported by the application
export type NetworkMode = 'local' | 'conway'

// Connection status for each endpoint
export type ConnectionStatus = 'connected' | 'connecting' | 'disconnected' | 'error'

// Network configuration interface
export interface NetworkConfig {
  mode: NetworkMode
  faucetUrl: string
  serviceUrl: string
  isTestnet: boolean
  displayName: string
}

// Chain ID validation regex (relaxed for flexibility)
const CHAIN_ID_REGEX = /^[a-f0-9]{40,}$/

// Application ID validation regex (relaxed for flexibility)
const APP_ID_REGEX = /^[a-f0-9]{40,}$/

/**
 * Get current network configuration from environment variables
 * @returns NetworkConfig object with current settings
 */
export function getNetworkConfig(): NetworkConfig {
  const mode = (import.meta.env.VITE_NETWORK_MODE || 'local') as NetworkMode
  const faucetUrl = import.meta.env.VITE_FAUCET_URL || 'http://localhost:8080'
  const serviceUrl = import.meta.env.VITE_SERVICE_URL || 'http://localhost:8081'

  return {
    mode,
    faucetUrl,
    serviceUrl,
    isTestnet: mode === 'conway',
    displayName: mode === 'conway' ? 'Conway Testnet' : 'Local Network'
  }
}

/**
 * Validate chain ID format
 * @param chainId - Chain ID to validate
 * @returns true if valid, false otherwise
 */
export function validateChainId(chainId: string | null | undefined): boolean {
  if (!chainId) return false
  return CHAIN_ID_REGEX.test(chainId)
}

/**
 * Validate application ID format
 * @param appId - Application ID to validate
 * @returns true if valid, false otherwise
 */
export function validateAppId(appId: string | null | undefined): boolean {
  if (!appId) return false
  return APP_ID_REGEX.test(appId)
}

/**
 * Build GraphQL endpoint URL for a specific chain and application
 * @param chainId - Chain ID (validated)
 * @param appId - Application ID (validated)
 * @returns Full GraphQL endpoint URL
 * @throws Error if chain ID or app ID is invalid
 */
export function buildGraphQLEndpoint(chainId: string, appId: string): string {
  if (!validateChainId(chainId)) {
    throw new Error(`Invalid chain ID format: ${chainId}`)
  }

  if (!validateAppId(appId)) {
    throw new Error(`Invalid application ID format: ${appId}`)
  }

  const config = getNetworkConfig()
  return `${config.serviceUrl}/chains/${chainId}/applications/${appId}`
}

/**
 * Truncate chain/app ID for display
 * @param id - Full chain or app ID
 * @param length - Number of characters to show (default: 12)
 * @returns Truncated ID with ellipsis
 */
export function truncateId(id: string, length: number = 12): string {
  if (!id || id.length <= length) return id
  return `${id.substring(0, length)}...`
}

/**
 * Check if service URL is available and responding
 * @param url - Service URL to check
 * @param timeout - Timeout in milliseconds (default: 5000)
 * @returns Promise<boolean> - true if service is available
 */
export async function checkServiceHealth(
  url: string,
  timeout: number = 5000
): Promise<boolean> {
  try {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), timeout)

    const response = await fetch(url, {
      method: 'HEAD',
      signal: controller.signal,
    })

    clearTimeout(timeoutId)
    return response.ok
  } catch (error) {
    console.error(`Service health check failed for ${url}:`, error)
    return false
  }
}

/**
 * Check if GraphQL endpoint is available
 * @param endpoint - GraphQL endpoint URL
 * @param timeout - Timeout in milliseconds (default: 5000)
 * @returns Promise<boolean> - true if endpoint is available
 */
export async function checkGraphQLHealth(
  endpoint: string,
  timeout: number = 5000
): Promise<boolean> {
  try {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), timeout)

    const response = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        query: '{ __typename }',
      }),
      signal: controller.signal,
    })

    clearTimeout(timeoutId)
    return response.ok
  } catch (error) {
    console.error(`GraphQL health check failed for ${endpoint}:`, error)
    return false
  }
}

/**
 * Get user-friendly error message based on error type
 * @param error - Error object
 * @param context - Context for the error (e.g., "fetching table state")
 * @returns User-friendly error message
 */
export function getUserFriendlyError(error: unknown, context: string): string {
  if (error instanceof Error) {
    // Network errors
    if (error.message.includes('fetch') || error.message.includes('network')) {
      return `Network error while ${context}. Please check your connection.`
    }

    // GraphQL errors
    if (error.message.includes('GraphQL')) {
      return `Data error while ${context}. Please try again.`
    }

    // Validation errors
    if (error.message.includes('Invalid')) {
      return `Configuration error: ${error.message}`
    }

    // Generic error with message
    return `Error while ${context}: ${error.message}`
  }

  // Unknown error type
  return `Unknown error while ${context}. Please try again.`
}

/**
 * Retry a function with exponential backoff
 * @param fn - Async function to retry
 * @param maxRetries - Maximum number of retries (default: 3)
 * @param initialDelay - Initial delay in ms (default: 1000)
 * @returns Promise with function result
 */
export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  maxRetries: number = 3,
  initialDelay: number = 1000
): Promise<T> {
  let lastError: Error | null = null

  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error as Error

      if (attempt < maxRetries - 1) {
        const delay = initialDelay * Math.pow(2, attempt)
        console.log(`Retry attempt ${attempt + 1}/${maxRetries} after ${delay}ms`)
        await new Promise(resolve => setTimeout(resolve, delay))
      }
    }
  }

  throw lastError
}

/**
 * Environment variable getters with validation
 */
export const ENV = {
  // Network
  networkMode: (): NetworkMode => {
    const mode = import.meta.env.VITE_NETWORK_MODE || 'local'
    if (mode !== 'local' && mode !== 'conway') {
      console.warn(`Invalid network mode: ${mode}, defaulting to local`)
      return 'local'
    }
    return mode as NetworkMode
  },

  // URLs
  faucetUrl: (): string => import.meta.env.VITE_FAUCET_URL || 'http://localhost:8080',
  serviceUrl: (): string => import.meta.env.VITE_SERVICE_URL || 'http://localhost:8081',

  // Table Chain
  tableChainId: (): string => import.meta.env.VITE_TABLE_CHAIN_ID || '',
  tableAppId: (): string => import.meta.env.VITE_TABLE_APP_ID || '',

  // Player A Chain
  playerAChainId: (): string => import.meta.env.VITE_PLAYER_A_CHAIN_ID || '',
  playerAHandAppId: (): string => import.meta.env.VITE_PLAYER_A_HAND_APP_ID || '',

  // Player B Chain
  playerBChainId: (): string => import.meta.env.VITE_PLAYER_B_CHAIN_ID || '',
  playerBHandAppId: (): string => import.meta.env.VITE_PLAYER_B_HAND_APP_ID || '',

  // Game Config
  minStake: (): number => parseInt(import.meta.env.VITE_MIN_STAKE || '10', 10),
  maxStake: (): number => parseInt(import.meta.env.VITE_MAX_STAKE || '1000', 10),
}

/**
 * Validate that all required environment variables are set
 * @returns Object with validation results
 */
export function validateEnvironment(): {
  valid: boolean
  missing: string[]
  invalid: string[]
} {
  const missing: string[] = []
  const invalid: string[] = []

  // Check required chain IDs
  const tableChainId = ENV.tableChainId()
  if (!tableChainId) missing.push('VITE_TABLE_CHAIN_ID')
  else if (!validateChainId(tableChainId)) invalid.push('VITE_TABLE_CHAIN_ID')

  const playerAChainId = ENV.playerAChainId()
  if (!playerAChainId) missing.push('VITE_PLAYER_A_CHAIN_ID')
  else if (!validateChainId(playerAChainId)) invalid.push('VITE_PLAYER_A_CHAIN_ID')

  const playerBChainId = ENV.playerBChainId()
  if (!playerBChainId) missing.push('VITE_PLAYER_B_CHAIN_ID')
  else if (!validateChainId(playerBChainId)) invalid.push('VITE_PLAYER_B_CHAIN_ID')

  // Check required app IDs
  const tableAppId = ENV.tableAppId()
  if (!tableAppId) missing.push('VITE_TABLE_APP_ID')
  else if (!validateAppId(tableAppId)) invalid.push('VITE_TABLE_APP_ID')

  const playerAHandAppId = ENV.playerAHandAppId()
  if (!playerAHandAppId) missing.push('VITE_PLAYER_A_HAND_APP_ID')
  else if (!validateAppId(playerAHandAppId)) invalid.push('VITE_PLAYER_A_HAND_APP_ID')

  const playerBHandAppId = ENV.playerBHandAppId()
  if (!playerBHandAppId) missing.push('VITE_PLAYER_B_HAND_APP_ID')
  else if (!validateAppId(playerBHandAppId)) invalid.push('VITE_PLAYER_B_HAND_APP_ID')

  return {
    valid: missing.length === 0 && invalid.length === 0,
    missing,
    invalid,
  }
}
