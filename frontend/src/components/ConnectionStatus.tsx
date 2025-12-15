/**
 * ConnectionStatus Component
 *
 * Displays the current network mode and connection status for all chains
 * Provides visual feedback for local vs Conway testnet connections
 */

import { Wifi, WifiOff, Loader2, Globe, Monitor } from 'lucide-react'
import type { NetworkConfig, ConnectionStatus as Status } from '../config/network'

interface ConnectionStatusProps {
  networkConfig: NetworkConfig
  connectionStatus: {
    table: Status
    playerA: Status
    playerB: Status
  }
}

// Status indicator component
function StatusIndicator({ name, status }: { name: string; status: Status }) {
  const getStatusIcon = () => {
    switch (status) {
      case 'connected':
        return <Wifi className="w-3 h-3 text-[var(--emerald)]" />
      case 'connecting':
        return <Loader2 className="w-3 h-3 text-[var(--gold)] animate-spin" />
      case 'disconnected':
        return <WifiOff className="w-3 h-3 text-gray-500" />
      case 'error':
        return <WifiOff className="w-3 h-3 text-[var(--crimson)]" />
      default:
        return <WifiOff className="w-3 h-3 text-gray-500" />
    }
  }

  const getStatusColor = () => {
    switch (status) {
      case 'connected':
        return 'text-[var(--emerald)]'
      case 'connecting':
        return 'text-[var(--gold)]'
      case 'disconnected':
        return 'text-gray-500'
      case 'error':
        return 'text-[var(--crimson)]'
      default:
        return 'text-gray-500'
    }
  }

  const getStatusText = () => {
    switch (status) {
      case 'connected':
        return 'Connected'
      case 'connecting':
        return 'Connecting...'
      case 'disconnected':
        return 'Disconnected'
      case 'error':
        return 'Error'
      default:
        return 'Unknown'
    }
  }

  return (
    <div className="flex items-center gap-2 px-2 py-1 rounded bg-[var(--obsidian)]/50">
      {getStatusIcon()}
      <span className="text-xs text-gray-400">{name}:</span>
      <span className={`text-xs font-medium ${getStatusColor()}`}>
        {getStatusText()}
      </span>
    </div>
  )
}

export function ConnectionStatus({
  networkConfig,
  connectionStatus,
}: ConnectionStatusProps) {
  const allConnected =
    connectionStatus.table === 'connected' &&
    connectionStatus.playerA === 'connected' &&
    connectionStatus.playerB === 'connected'

  const anyError =
    connectionStatus.table === 'error' ||
    connectionStatus.playerA === 'error' ||
    connectionStatus.playerB === 'error'

  return (
    <div className="mb-6 p-4 rounded-xl border border-white/5 bg-[var(--abyss)]/50 backdrop-blur-sm">
      <div className="flex items-center justify-between flex-wrap gap-4">
        {/* Network Mode Badge */}
        <div className="flex items-center gap-3">
          <div
            className="flex items-center gap-2 px-4 py-2 rounded-lg font-medium"
            style={{
              background: networkConfig.isTestnet
                ? 'linear-gradient(135deg, rgba(0, 229, 255, 0.1), rgba(0, 153, 255, 0.1))'
                : 'linear-gradient(135deg, rgba(139, 92, 246, 0.1), rgba(99, 102, 241, 0.1))',
              border: `1px solid ${
                networkConfig.isTestnet
                  ? 'rgba(0, 229, 255, 0.2)'
                  : 'rgba(139, 92, 246, 0.2)'
              }`,
            }}
          >
            {networkConfig.isTestnet ? (
              <Globe className="w-4 h-4 text-[var(--cyan)]" />
            ) : (
              <Monitor className="w-4 h-4 text-purple-400" />
            )}
            <span
              className="text-sm"
              style={{
                color: networkConfig.isTestnet ? 'var(--cyan)' : '#a78bfa',
              }}
            >
              {networkConfig.displayName}
            </span>
          </div>

          {/* Overall Status Indicator */}
          {allConnected && (
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[var(--emerald)]/10 border border-[var(--emerald)]/20">
              <div className="w-2 h-2 rounded-full bg-[var(--emerald)] animate-pulse" />
              <span className="text-xs text-[var(--emerald)] font-medium">
                All Systems Online
              </span>
            </div>
          )}

          {anyError && (
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[var(--crimson)]/10 border border-[var(--crimson)]/20">
              <div className="w-2 h-2 rounded-full bg-[var(--crimson)] animate-pulse" />
              <span className="text-xs text-[var(--crimson)] font-medium">
                Connection Issues
              </span>
            </div>
          )}
        </div>

        {/* Chain Connection Status */}
        <div className="flex items-center gap-2 flex-wrap">
          <StatusIndicator name="Table" status={connectionStatus.table} />
          <StatusIndicator name="Player A" status={connectionStatus.playerA} />
          <StatusIndicator name="Player B" status={connectionStatus.playerB} />
        </div>
      </div>

      {/* Service URL Info (for debugging) */}
      {networkConfig.isTestnet && (
        <div className="mt-3 pt-3 border-t border-white/5">
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500 font-mono">Service:</span>
            <span className="text-xs text-gray-400 font-mono">
              {networkConfig.serviceUrl}
            </span>
          </div>
        </div>
      )}
    </div>
  )
}
