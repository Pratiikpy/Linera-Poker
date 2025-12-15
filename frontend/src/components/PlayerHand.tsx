import { HandState } from '../types'
import { Card, CardBack, CardPlaceholder } from './Card'
import { Eye, EyeOff, Shield, Trophy, TrendingDown } from 'lucide-react'

interface PlayerHandProps {
  player: 'A' | 'B'
  handState: HandState | null
  isCurrentPlayer: boolean
  isMyTurn: boolean
}

export function PlayerHand({ player, handState, isCurrentPlayer, isMyTurn }: PlayerHandProps) {
  const hasCards = handState?.hole_cards && handState.hole_cards.length > 0
  const chainColor = player === 'A' ? 'var(--chain-player-a)' : 'var(--chain-player-b)'

  return (
    <div className={isMyTurn && isCurrentPlayer ? 'your-turn rounded-xl' : ''}>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div
            className="w-10 h-10 rounded-xl flex items-center justify-center text-white font-bold"
            style={{
              background: chainColor,
              boxShadow: `0 0 20px ${chainColor}40`,
              fontFamily: 'Bebas Neue',
              fontSize: '1.25rem',
            }}
          >
            {player}
          </div>
          <div>
            <h3 className="text-lg font-bold text-white" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
              PLAYER {player}
            </h3>
            <p className="text-xs text-gray-500 font-mono">
              {player === 'A' ? 'chain-a' : 'chain-b'}
            </p>
          </div>
        </div>

        {isCurrentPlayer ? (
          <span className="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-full bg-[var(--emerald)]/10 text-[var(--emerald)] border border-[var(--emerald)]/30">
            <Eye className="w-3.5 h-3.5" />
            YOUR VIEW
          </span>
        ) : (
          <span className="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-full bg-[var(--crimson)]/10 text-[var(--crimson)] border border-[var(--crimson)]/30">
            <EyeOff className="w-3.5 h-3.5" />
            HIDDEN
          </span>
        )}
      </div>

      {/* Privacy Notice - Chain Shield */}
      <div className="chain-shield mb-5">
        <div className="flex items-center gap-2 text-xs relative z-10">
          <Shield className="w-4 h-4 text-[var(--cyan)]" />
          <span className="text-gray-400">
            Cards stored on <strong className="text-white">Player {player}'s chain</strong>.
            {isCurrentPlayer ? (
              <span className="text-[var(--emerald)] ml-1">Only you can see them!</span>
            ) : (
              <span className="text-[var(--crimson)] ml-1">Dealer cannot access this state.</span>
            )}
          </span>
        </div>
      </div>

      {/* Hole Cards */}
      <div className="flex justify-center gap-4 mb-5">
        {hasCards && isCurrentPlayer ? (
          // Show actual cards to current player with active player glow
          handState.hole_cards!.map((card, index) => (
            <Card
              key={index}
              card={card}
              dealt
              index={index}
              isActivePlayer={isMyTurn}
            />
          ))
        ) : hasCards ? (
          // Show card backs to other players with mystery animation
          Array.from({ length: handState.hole_cards!.length }).map((_, index) => (
            <CardBack key={index} dealt index={index} mystery />
          ))
        ) : (
          // No cards yet
          Array.from({ length: 2 }).map((_, index) => (
            <CardPlaceholder key={index} />
          ))
        )}
      </div>

      {/* Game Status */}
      <div className="grid grid-cols-2 gap-3 text-center">
        <div className="p-3 rounded-lg" style={{ background: 'rgba(10, 10, 18, 0.6)' }}>
          <div className="text-xs text-gray-500 mb-1 font-mono uppercase tracking-wider">Game ID</div>
          <div className="text-white font-medium" style={{ fontFamily: 'JetBrains Mono' }}>
            {handState?.game_id ?? '—'}
          </div>
        </div>
        <div className="p-3 rounded-lg" style={{ background: 'rgba(10, 10, 18, 0.6)' }}>
          <div className="text-xs text-gray-500 mb-1 font-mono uppercase tracking-wider">My Turn</div>
          <div
            className={`font-bold ${isMyTurn ? 'text-[var(--gold-bright)]' : 'text-gray-600'}`}
            style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.1em' }}
          >
            {isMyTurn ? 'YES' : 'NO'}
          </div>
        </div>
      </div>

      {/* Current Bet to Match */}
      <div className="mt-4 p-4 rounded-lg text-center" style={{ background: 'rgba(10, 10, 18, 0.6)' }}>
        <div className="text-xs text-gray-500 mb-1 font-mono uppercase tracking-wider">Bet to Match</div>
        <div className="text-2xl font-bold text-white" style={{ fontFamily: 'Bebas Neue' }}>
          {parseInt(handState?.current_bet || '0').toLocaleString()}
        </div>
      </div>

      {/* Game Result */}
      {handState?.game_result && (
        <div
          className={`mt-4 p-5 rounded-xl border text-center ${
            handState.game_result.won
              ? 'bg-[var(--emerald)]/10 border-[var(--emerald)]/30'
              : 'bg-[var(--crimson)]/10 border-[var(--crimson)]/30'
          }`}
        >
          <div className="flex items-center justify-center gap-2 mb-2">
            {handState.game_result.won ? (
              <Trophy className="w-8 h-8 text-[var(--gold-bright)]" />
            ) : (
              <TrendingDown className="w-8 h-8 text-[var(--crimson)]" />
            )}
          </div>
          <div
            className={`text-2xl font-bold ${
              handState.game_result.won ? 'text-[var(--emerald)]' : 'text-[var(--crimson)]'
            }`}
            style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.1em' }}
          >
            {handState.game_result.won ? 'YOU WON!' : 'YOU LOST'}
          </div>
          <div className="text-lg text-white mt-1" style={{ fontFamily: 'JetBrains Mono' }}>
            Payout: {parseInt(handState.game_result.payout || '0').toLocaleString()}
          </div>
        </div>
      )}
    </div>
  )
}
