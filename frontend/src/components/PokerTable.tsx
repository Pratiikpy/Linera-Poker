import { useState, useEffect } from 'react'
import { TableState } from '../types'
import { Card, CardPlaceholder } from './Card'
import { Trophy, Sparkles } from 'lucide-react'

interface PokerTableProps {
  tableState: TableState | null
  currentPlayer: 'A' | 'B'
  onStartNewGame: () => Promise<void>
}

export function PokerTable({ tableState, currentPlayer: _currentPlayer, onStartNewGame }: PokerTableProps) {
  const [previousPot, setPreviousPot] = useState('0')
  const [potUpdating, setPotUpdating] = useState(false)
  const [confettiParticles, setConfettiParticles] = useState<Array<{ id: number; left: number; delay: number; color: string }>>([])

  const formatAmount = (amount: string) => {
    const num = parseInt(amount || '0')
    return num.toLocaleString()
  }

  // Trigger pot update animation
  useEffect(() => {
    const currentPot = tableState?.pot || '0'
    if (currentPot !== previousPot && parseInt(currentPot) > parseInt(previousPot)) {
      setPotUpdating(true)
      setTimeout(() => setPotUpdating(false), 600)
    }
    setPreviousPot(currentPot)
  }, [tableState?.pot, previousPot])

  // Generate confetti on winner
  useEffect(() => {
    if (tableState?.winner && confettiParticles.length === 0) {
      const particles = Array.from({ length: 30 }, (_, i) => ({
        id: i,
        left: Math.random() * 100,
        delay: Math.random() * 0.5,
        color: i % 2 === 0 ? 'confetti-gold' : 'confetti-cyan'
      }))
      setConfettiParticles(particles)
    } else if (!tableState?.winner) {
      setConfettiParticles([])
    }
  }, [tableState?.winner, confettiParticles.length])

  const getPhaseInfo = (phase: string) => {
    switch (phase) {
      case 'WaitingForPlayers':
        return { label: 'WAITING', badgeClass: 'badge-waiting' }
      case 'Dealing':
        return { label: 'DEALING', badgeClass: 'badge-dealing' }
      case 'PreFlop':
        return { label: 'PRE-FLOP', badgeClass: 'badge-betting' }
      case 'Flop':
        return { label: 'FLOP', badgeClass: 'badge-betting' }
      case 'Turn':
        return { label: 'TURN', badgeClass: 'badge-betting' }
      case 'River':
        return { label: 'RIVER', badgeClass: 'badge-betting' }
      case 'Showdown':
        return { label: 'SHOWDOWN', badgeClass: 'badge-showdown' }
      case 'Settlement':
        return { label: 'SETTLEMENT', badgeClass: 'badge-finished' }
      case 'Finished':
        return { label: 'FINISHED', badgeClass: 'badge-finished' }
      default:
        return { label: phase.toUpperCase(), badgeClass: 'badge-waiting' }
    }
  }

  const phase = getPhaseInfo(tableState?.phase || 'WaitingForPlayers')

  return (
    <div className="poker-table p-8 relative min-h-[320px]">
      {/* Phase Badge */}
      <div className="absolute top-4 left-1/2 -translate-x-1/2 z-10">
        <span className={`status-badge ${phase.badgeClass}`}>
          {phase.label}
        </span>
      </div>

      {/* Pot Display with Animation */}
      <div className="flex justify-center mb-8 mt-8">
        <div className={`pot-display flex items-center gap-4 ${potUpdating ? 'pot-display-updating' : ''}`}>
          <div className="chip chip-gold">$</div>
          <div className="text-center">
            <div className={`pot-amount ${potUpdating ? 'pot-amount-updating' : ''}`}>
              {formatAmount(tableState?.pot || '0')}
            </div>
          </div>
        </div>
      </div>

      {/* Community Cards with Shimmer */}
      <div className="flex justify-center gap-3 mb-8">
        {tableState?.community_cards && tableState.community_cards.length > 0 ? (
          tableState.community_cards.map((card, index) => {
            const shouldShimmer = ['Flop', 'Turn', 'River'].includes(tableState.phase)
            return (
              <Card
                key={index}
                card={card}
                shimmer={shouldShimmer}
                index={index}
              />
            )
          })
        ) : (
          // Placeholder cards
          Array.from({ length: 5 }).map((_, index) => (
            <CardPlaceholder key={index} />
          ))
        )}
      </div>

      {/* Betting Info */}
      <div className="flex justify-center gap-6">
        <div className="text-center">
          <div className="text-xs text-gray-500 uppercase tracking-wider mb-1" style={{ fontFamily: 'JetBrains Mono' }}>
            Current Bet
          </div>
          <div className="text-lg font-bold text-white" style={{ fontFamily: 'Bebas Neue' }}>
            {formatAmount(tableState?.current_bet || '0')}
          </div>
        </div>
        <div className="text-center">
          <div className="text-xs text-gray-500 uppercase tracking-wider mb-1" style={{ fontFamily: 'JetBrains Mono' }}>
            Min Raise
          </div>
          <div className="text-lg font-bold text-white" style={{ fontFamily: 'Bebas Neue' }}>
            {formatAmount(tableState?.min_raise || '0')}
          </div>
        </div>
        {/* Blind Structure */}
        <div className="text-center px-3 py-1 rounded-lg bg-white/5 border border-white/10">
          <div className="text-xs text-gray-500 uppercase tracking-wider mb-1" style={{ fontFamily: 'JetBrains Mono' }}>
            Blinds
          </div>
          <div className="text-sm font-bold" style={{ fontFamily: 'Bebas Neue' }}>
            <span className="text-[var(--gold)]">{formatAmount(tableState?.small_blind || '5')}</span>
            <span className="text-gray-500 mx-1">/</span>
            <span className="text-[var(--cyan)]">{formatAmount(tableState?.big_blind || '10')}</span>
          </div>
        </div>
      </div>

      {/* Turn Indicator */}
      {tableState?.turn_seat && (
        <div className="absolute bottom-4 left-1/2 -translate-x-1/2">
          <div className="px-4 py-2 rounded-full bg-[var(--gold)]/20 border border-[var(--gold)]/50 your-turn">
            <span className="text-sm font-medium text-[var(--gold-bright)]" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.1em' }}>
              {tableState.turn_seat === 'Player1' ? "PLAYER A'S TURN" : "PLAYER B'S TURN"}
            </span>
          </div>
        </div>
      )}

      {/* Winner / Split Pot Overlay */}
      {((tableState?.winner || ['Settlement', 'Finished'].includes(tableState?.phase || ''))) && (
        <div className="winner-overlay">
          {/* Confetti Particles - Only for winner, maybe different for split? */}
          {confettiParticles.map((particle) => (
            <div
              key={particle.id}
              className={`confetti-particle ${particle.color}`}
              style={{
                left: `${particle.left}%`,
                top: '0',
                animationDelay: `${particle.delay}s`
              }}
            />
          ))}

          <div className="text-center relative z-10 flex flex-col items-center gap-6">
            <Trophy className={`w-20 h-20 mb-2 ${tableState?.winner ? 'text-[var(--gold-bright)]' : 'text-[var(--cyan)]'} trophy-animated`} />

            <div className="winner-text flex items-center justify-center gap-4">
              <Sparkles className="w-10 h-10" />
              <span className="leading-none pt-2">
                {tableState?.winner === 'Player1' ? 'PLAYER A WINS!' :
                  tableState?.winner === 'Player2' ? 'PLAYER B WINS!' :
                    'SPLIT POT'}
              </span>
              <Sparkles className="w-10 h-10" />
            </div>

            <div className={`text-4xl font-bold tracking-wider ${tableState?.winner ? 'text-[var(--emerald)]' : 'text-[var(--cyan)]'}`} style={{ fontFamily: 'Bebas Neue', textShadow: '0 0 20px rgba(0,0,0,0.5)' }}>
              {tableState?.winner
                ? `+${formatAmount(tableState.pot || '0')}`
                : `POT SPLIT: ${formatAmount(tableState?.pot || '0')}`
              }
            </div>

            <button
              onClick={onStartNewGame}
              className="mt-6 px-8 py-3 rounded-xl text-xl font-bold transition-all duration-300 hover:scale-105 hover:shadow-lg active:scale-95"
              style={{
                fontFamily: 'Bebas Neue',
                background: 'var(--gradient-gold)',
                color: '#1a0f0a',
                boxShadow: '0 0 20px rgba(212, 160, 25, 0.4)'
              }}
            >
              START NEW GAME
            </button>
          </div>
        </div>
      )}

      {/* Player Seats */}
      <div className="absolute bottom-16 left-6">
        <PlayerSeat
          player="A"
          players={tableState?.players || []}
          isActive={tableState?.turn_seat === 'Player1'}
          isDealer={tableState?.dealer_button === 'Player1'}
        />
      </div>
      <div className="absolute bottom-16 right-6">
        <PlayerSeat
          player="B"
          players={tableState?.players || []}
          isActive={tableState?.turn_seat === 'Player2'}
          isDealer={tableState?.dealer_button === 'Player2'}
        />
      </div>
    </div>
  )
}

interface PlayerSeatProps {
  player: 'A' | 'B'
  players: { seat: string; stake: string; has_folded: boolean; current_bet: string }[]
  isActive: boolean
  isDealer: boolean
}

function PlayerSeat({ player, players, isActive, isDealer }: PlayerSeatProps) {
  const seatId = player === 'A' ? 'Player1' : 'Player2'
  const playerInfo = players.find(p => p.seat === seatId)
  const chainColor = player === 'A' ? 'var(--chain-player-a)' : 'var(--chain-player-b)'

  return (
    <div
      className={`rounded-xl p-3 backdrop-blur-sm transition-all duration-300 relative ${isActive ? 'ring-2' : ''
        }`}
      style={{
        background: 'rgba(10, 10, 18, 0.8)',
        borderColor: isActive ? chainColor : 'transparent',
        boxShadow: isActive ? `0 0 20px ${chainColor}40` : 'none',
        ['--tw-ring-color' as string]: chainColor,
      }}
    >
      {/* Dealer Button */}
      {isDealer && (
        <div
          className="absolute -top-2 -right-2 w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold shadow-lg"
          style={{
            background: 'var(--gradient-gold)',
            color: '#1a0f0a',
            boxShadow: '0 0 10px var(--gold)',
          }}
          title="Dealer Button (Small Blind)"
        >
          D
        </div>
      )}
      <div className="flex items-center gap-2 mb-1">
        <div
          className="w-2.5 h-2.5 rounded-full"
          style={{
            background: playerInfo ? chainColor : 'var(--steel)',
            boxShadow: playerInfo ? `0 0 8px ${chainColor}` : 'none',
          }}
        />
        <span className="text-white text-sm font-medium" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
          PLAYER {player}
        </span>
        {/* Blind indicator */}
        {playerInfo && isDealer && (
          <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--gold)]/20 text-[var(--gold)]">SB</span>
        )}
        {playerInfo && !isDealer && (
          <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--cyan)]/20 text-[var(--cyan)]">BB</span>
        )}
      </div>
      {playerInfo && (
        <div className="text-xs font-mono">
          {playerInfo.has_folded ? (
            <span className="text-[var(--crimson)]">FOLDED</span>
          ) : (
            <span className="text-gray-400">
              Bet: <span className="text-white">{parseInt(playerInfo.current_bet || '0').toLocaleString()}</span>
            </span>
          )}
        </div>
      )}
    </div>
  )
}
