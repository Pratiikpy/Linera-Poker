import { useState } from 'react'
import { TableState, HandState, BetAction } from '../types'
import { Coins, Eye, Loader2, Sparkles } from 'lucide-react'

interface GameControlsProps {
  tableState: TableState | null
  currentPlayer: 'A' | 'B'
  playerState: HandState | null
  onJoinTable: (player: 'A' | 'B', stake: number) => Promise<void>
  onBet: (player: 'A' | 'B', action: BetAction) => Promise<void>
  onReveal: (player: 'A' | 'B') => Promise<void>
  loading: boolean
}

export function GameControls({
  tableState,
  currentPlayer,
  playerState: _playerState,
  onJoinTable,
  onBet,
  onReveal,
  loading,
}: GameControlsProps) {
  const [stake, setStake] = useState(100)
  const [raiseAmount, setRaiseAmount] = useState(50)

  const phase = tableState?.phase || 'WaitingForPlayers'

  // Determine if it's my turn from TABLE state (not hand state, since we call table service directly)
  // turn_seat comes as "Player1" or "Player2" from GraphQL (Rust Seat enum Debug format)
  const myTurnSeat = currentPlayer === 'A' ? 'Player1' : 'Player2'
  const isMyTurn = tableState?.turn_seat === myTurnSeat

  // Check if player has joined by looking at table players list
  const hasJoined = tableState?.players?.some(
    p => p.chain_id === (currentPlayer === 'A'
      ? import.meta.env.VITE_PLAYER_A_CHAIN_ID
      : import.meta.env.VITE_PLAYER_B_CHAIN_ID)
  ) || false

  // Determine which actions are available
  const canJoin = phase === 'WaitingForPlayers' && !hasJoined
  const canBet = isMyTurn && ['PreFlop', 'Flop', 'Turn', 'River'].includes(phase)

  // Check if player needs to reveal (in Showdown/Revealing phase and hasn't revealed yet)
  const myPlayer = tableState?.players?.find(
    p => p.chain_id === (currentPlayer === 'A'
      ? import.meta.env.VITE_PLAYER_A_CHAIN_ID
      : import.meta.env.VITE_PLAYER_B_CHAIN_ID)
  )
  const canReveal = ['Showdown', 'Revealing'].includes(phase) && myPlayer && !myPlayer.has_revealed

  const handleJoin = async () => {
    if (loading) return
    await onJoinTable(currentPlayer, stake)
  }

  const handleBet = async (action: BetAction) => {
    if (loading || !isMyTurn) return
    await onBet(currentPlayer, action)
  }

  const handleReveal = async () => {
    if (loading) return
    await onReveal(currentPlayer)
  }

  return (
    <div className="mt-6 p-6 rounded-xl" style={{ background: 'linear-gradient(135deg, rgba(18, 18, 31, 0.9) 0%, rgba(10, 10, 18, 0.95) 100%)', border: '1px solid rgba(255, 255, 255, 0.05)' }}>
      <h3 className="text-xl font-bold text-white mb-5 flex items-center gap-3" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.05em' }}>
        <Sparkles className="w-5 h-5 text-[var(--gold)]" />
        GAME CONTROLS
        {loading && <Loader2 className="w-4 h-4 animate-spin text-gray-400" />}
      </h3>

      {/* Join Table */}
      {canJoin && (
        <div className="mb-6">
          <label className="block text-xs text-gray-500 mb-2 font-mono uppercase tracking-wider">
            Stake Amount
          </label>
          <div className="flex gap-3">
            <input
              type="number"
              value={stake}
              onChange={(e) => setStake(parseInt(e.target.value) || 0)}
              min={parseInt(tableState?.min_stake || '10')}
              max={parseInt(tableState?.max_stake || '1000')}
              className="input-stakes flex-1"
            />
            <button
              onClick={handleJoin}
              disabled={loading}
              className="btn-poker btn-call flex items-center gap-2"
            >
              <Coins className="w-4 h-4" />
              JOIN TABLE
            </button>
          </div>
          <p className="text-xs text-gray-600 mt-2 font-mono">
            Range: {tableState?.min_stake || '10'} — {tableState?.max_stake || '1000'}
          </p>
        </div>
      )}

      {/* Betting Actions */}
      {canBet && (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => handleBet({ Check: null })}
              disabled={loading}
              className="btn-poker btn-check"
            >
              CHECK
            </button>
            <button
              onClick={() => handleBet({ Call: null })}
              disabled={loading}
              className="btn-poker btn-call"
            >
              CALL
            </button>
          </div>

          <div className="flex gap-3">
            <input
              type="number"
              value={raiseAmount}
              onChange={(e) => setRaiseAmount(parseInt(e.target.value) || 0)}
              min={parseInt(tableState?.min_raise || '10')}
              className="input-stakes flex-1"
            />
            <button
              onClick={() => handleBet({ Raise: raiseAmount.toString() })}
              disabled={loading}
              className="btn-poker btn-raise"
            >
              RAISE
            </button>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => handleBet({ AllIn: null })}
              disabled={loading}
              className="btn-poker btn-allin"
            >
              ALL IN
            </button>
            <button
              onClick={() => handleBet({ Fold: null })}
              disabled={loading}
              className="btn-poker btn-fold"
            >
              FOLD
            </button>
          </div>
        </div>
      )}

      {/* Reveal Cards */}
      {canReveal && (
        <div>
          <button
            onClick={handleReveal}
            disabled={loading}
            className="w-full btn-poker btn-reveal flex items-center justify-center gap-2"
          >
            <Eye className="w-5 h-5" />
            REVEAL MY CARDS
          </button>
          <p className="text-xs text-gray-600 mt-2 text-center font-mono">
            Send your hole cards to the dealer for showdown
          </p>
        </div>
      )}

      {/* Waiting Message */}
      {hasJoined && !canBet && !canReveal && (
        <div className="text-center py-6">
          {isMyTurn ? (
            <div className="flex items-center justify-center gap-2">
              <span className="inline-block w-2 h-2 rounded-full bg-[var(--gold)] animate-pulse" />
              <span className="text-[var(--gold-bright)] font-bold" style={{ fontFamily: 'Bebas Neue', letterSpacing: '0.1em' }}>
                IT'S YOUR TURN!
              </span>
            </div>
          ) : (
            <span className="text-gray-500 text-sm">
              Waiting for {currentPlayer === 'A' ? 'Player B' : 'Player A'}...
            </span>
          )}
        </div>
      )}

      {/* Status Info */}
      <div className="mt-6 pt-4 border-t border-white/5">
        <div className="grid grid-cols-2 gap-4 text-xs">
          <div>
            <span className="text-gray-600">Phase: </span>
            <span className="text-white font-medium">{phase}</span>
          </div>
          <div>
            <span className="text-gray-600">Turn Seat: </span>
            <span className="text-yellow-400 font-medium">{tableState?.turn_seat || 'null'}</span>
          </div>
          <div>
            <span className="text-gray-600">Your Turn: </span>
            <span className={isMyTurn ? 'text-[var(--gold-bright)] font-bold' : 'text-gray-500'}>
              {isMyTurn ? 'YES' : 'No'}
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}
