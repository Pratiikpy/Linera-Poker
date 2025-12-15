import { useState, useEffect } from 'react'
import { TableState } from '../types'
import { Network, Shield, ArrowRight, CheckCircle2, Radio } from 'lucide-react'

interface CrossChainInfoProps {
  tableState: TableState | null
}

export function CrossChainInfo({ tableState }: CrossChainInfoProps) {
  const [messageStatus, setMessageStatus] = useState<'idle' | 'sending' | 'confirmed'>('idle')

  const tableChainId = import.meta.env.VITE_TABLE_CHAIN_ID || 'Not configured'
  const playerAChainId = import.meta.env.VITE_PLAYER_A_CHAIN_ID || 'Not configured'
  const playerBChainId = import.meta.env.VITE_PLAYER_B_CHAIN_ID || 'Not configured'

  const truncateId = (id: string) => {
    if (id.length > 16) {
      return `${id.slice(0, 6)}...${id.slice(-6)}`
    }
    return id
  }

  // Simulate message flow animation based on game phase
  useEffect(() => {
    const phase = tableState?.phase
    if (phase && ['Dealing', 'PreFlop', 'Flop', 'Turn', 'River'].includes(phase)) {
      setMessageStatus('sending')
      const timer = setTimeout(() => setMessageStatus('confirmed'), 1500)
      return () => clearTimeout(timer)
    } else {
      setMessageStatus('idle')
    }
  }, [tableState?.phase])

  return (
    <div className="rounded-xl p-6" style={{ background: 'linear-gradient(135deg, rgba(18, 18, 31, 0.8) 0%, rgba(10, 10, 18, 0.9) 100%)', border: '1px solid rgba(255, 255, 255, 0.05)' }}>
      <div className="flex items-center gap-3 mb-6">
        <div className="w-10 h-10 rounded-xl bg-[var(--cyan)]/10 flex items-center justify-center">
          <Network className="w-5 h-5 text-[var(--cyan)]" />
        </div>
        <div>
          <h2 className="text-xl font-bold text-white" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
            CROSS-CHAIN ARCHITECTURE
          </h2>
          <p className="text-xs text-gray-500 font-mono">Three independent microchains</p>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-4">
        {/* Table Chain */}
        <div className="chain-card chain-card-dealer">
          <div className="flex items-center gap-2 mb-3">
            <div className="chain-indicator connection-strong" style={{ background: 'var(--chain-dealer)', boxShadow: '0 0 10px var(--chain-dealer)' }} />
            <h3 className="font-medium text-white text-sm" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
              DEALER CHAIN
            </h3>
          </div>
          <div className="text-xs text-gray-500 font-mono mb-3 truncate">
            {truncateId(tableChainId)}
          </div>
          <div className="space-y-1.5 text-xs">
            <div className="flex justify-between">
              <span className="text-gray-500">Game ID</span>
              <span className="text-white font-mono">{tableState?.game_id ?? '—'}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">Phase</span>
              <span className="text-white">{tableState?.phase || '—'}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-500">Pot</span>
              <span className="text-[var(--gold-bright)] font-mono">{parseInt(tableState?.pot || '0').toLocaleString()}</span>
            </div>
          </div>
          <div className="mt-4 p-2.5 rounded-lg bg-[var(--crimson)]/10 border border-[var(--crimson)]/20">
            <div className="flex items-center gap-2 text-xs text-[var(--crimson)]">
              <Shield className="w-3.5 h-3.5" />
              <span>Cannot see player cards!</span>
            </div>
          </div>
        </div>

        {/* Player A Chain */}
        <div className="chain-card chain-card-a">
          <div className="flex items-center gap-2 mb-3">
            <div className="chain-indicator connection-strong" style={{ background: 'var(--chain-player-a)', boxShadow: '0 0 10px var(--chain-player-a)' }} />
            <h3 className="font-medium text-white text-sm" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
              PLAYER A CHAIN
            </h3>
          </div>
          <div className="text-xs text-gray-500 font-mono mb-3 truncate">
            {truncateId(playerAChainId)}
          </div>
          <div className="mt-4 p-2.5 rounded-lg bg-[var(--chain-player-a)]/10 border border-[var(--chain-player-a)]/20">
            <div className="flex items-center gap-2 text-xs text-[var(--chain-player-a)]">
              <Shield className="w-3.5 h-3.5" />
              <span>Private hole cards stored here</span>
            </div>
          </div>
        </div>

        {/* Player B Chain */}
        <div className="chain-card chain-card-b">
          <div className="flex items-center gap-2 mb-3">
            <div className="chain-indicator connection-strong" style={{ background: 'var(--chain-player-b)', boxShadow: '0 0 10px var(--chain-player-b)' }} />
            <h3 className="font-medium text-white text-sm" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
              PLAYER B CHAIN
            </h3>
          </div>
          <div className="text-xs text-gray-500 font-mono mb-3 truncate">
            {truncateId(playerBChainId)}
          </div>
          <div className="mt-4 p-2.5 rounded-lg bg-[var(--chain-player-b)]/10 border border-[var(--chain-player-b)]/20">
            <div className="flex items-center gap-2 text-xs text-[var(--chain-player-b)]">
              <Shield className="w-3.5 h-3.5" />
              <span>Private hole cards stored here</span>
            </div>
          </div>
        </div>
      </div>

      {/* Cross-Chain Message Flow with Animated Particles */}
      <div className="message-flow mt-6 flex-wrap justify-center relative">
        {/* Animated particles */}
        {messageStatus === 'sending' && (
          <>
            <div className="message-particle" style={{ left: '10%', top: '50%' }} />
            <div className="message-particle" style={{ left: '30%', top: '45%' }} />
            <div className="message-particle" style={{ left: '50%', top: '50%' }} />
          </>
        )}

        <span className="message-node" style={{ background: 'var(--chain-player-a)', color: 'white' }}>Player A</span>
        <ArrowRight className="message-arrow w-4 h-4" />
        <span className={`message-node bg-[var(--steel)] text-gray-300 ${messageStatus === 'sending' ? 'message-sending' : messageStatus === 'confirmed' ? 'message-confirmed' : ''}`}>
          {tableState?.phase || 'Action'}
        </span>
        <ArrowRight className="message-arrow w-4 h-4" />
        <span className="message-node" style={{ background: 'var(--chain-dealer)', color: '#1a0f0a' }}>Table</span>
        <ArrowRight className="message-arrow w-4 h-4" />
        <span className="message-node bg-[var(--steel)] text-gray-300">Response</span>
        <ArrowRight className="message-arrow w-4 h-4" />
        <span className="message-node" style={{ background: 'var(--chain-player-a)', color: 'white' }}>Player A</span>

        {/* Status indicator */}
        {messageStatus !== 'idle' && (
          <div className="absolute -right-8 top-1/2 -translate-y-1/2">
            {messageStatus === 'sending' ? (
              <Radio className="w-4 h-4 text-[var(--cyan)] animate-pulse" />
            ) : (
              <CheckCircle2 className="w-4 h-4 text-[var(--emerald)]" />
            )}
          </div>
        )}
      </div>

      <p className="text-xs text-gray-600 text-center mt-4 font-mono">
        All actions are cross-chain messages. No shared state access.
      </p>
    </div>
  )
}
