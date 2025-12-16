import { X } from 'lucide-react'

/**
 * JoiningTableLoading - Cross-chain message animation for player joining table
 *
 * Shows a modal overlay with animated cross-chain message flow:
 * - Player Chain (blue) → animated arrow → Table Chain (yellow)
 * - Gradient animated line showing message in transit
 * - Context about what's happening (joining, posting chips)
 *
 * Features:
 * - Modal overlay with backdrop blur
 * - Animated cross-chain message visualization
 * - Context-aware messaging
 * - Cancel button for user control
 * - Accessibility compliant
 * - Mobile responsive
 */

interface JoiningTableLoadingProps {
  /** Player identifier (A or B) */
  player: 'A' | 'B'
  /** Amount of chips being posted */
  chipAmount?: number
  /** Callback when user cancels */
  onCancel?: () => void
}

export function JoiningTableLoading({
  player,
  chipAmount = 100,
  onCancel,
}: JoiningTableLoadingProps) {
  const playerColor = player === 'A' ? '#4A90E2' : '#E24A90' // Blue for A, Pink for B
  const tableColor = '#F5A623' // Gold/Yellow for table

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4"
      role="dialog"
      aria-modal="true"
      aria-labelledby="joining-table-title"
      aria-describedby="joining-table-description"
    >
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onClick={onCancel}
        aria-hidden="true"
      />

      {/* Modal Content */}
      <div className="relative max-w-lg w-full bg-gradient-to-br from-gray-900 to-gray-800 rounded-2xl shadow-2xl border border-white/10 p-8">
        {/* Close Button */}
        {onCancel && (
          <button
            onClick={onCancel}
            className="absolute top-4 right-4 p-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 transition-colors"
            aria-label="Cancel joining table"
          >
            <X className="w-5 h-5 text-gray-400" />
          </button>
        )}

        {/* Title */}
        <h2
          id="joining-table-title"
          className="text-2xl font-bold text-white mb-2 text-center"
          style={{ fontFamily: 'Bebas Neue, sans-serif', letterSpacing: '0.1em' }}
        >
          JOINING TABLE
        </h2>

        {/* Description */}
        <p id="joining-table-description" className="text-gray-400 text-center mb-8">
          Player {player} posting {chipAmount} chips
        </p>

        {/* Cross-Chain Message Visualization */}
        <div className="relative py-12">
          {/* Player Chain (Source) */}
          <div className="absolute left-0 top-1/2 -translate-y-1/2">
            <div className="flex flex-col items-center gap-2">
              {/* Chain Circle */}
              <div
                className="w-20 h-20 rounded-full flex items-center justify-center shadow-xl relative"
                style={{
                  background: `linear-gradient(135deg, ${playerColor}, ${playerColor}dd)`,
                  boxShadow: `0 0 30px ${playerColor}66`,
                }}
              >
                {/* Pulsing ring */}
                <div
                  className="absolute inset-0 rounded-full animate-ping"
                  style={{ background: `${playerColor}40` }}
                />

                <span className="text-2xl font-bold text-white relative z-10">
                  {player}
                </span>
              </div>

              {/* Label */}
              <div className="text-center">
                <p className="text-xs font-medium text-white">Player Chain</p>
                <p className="text-[10px] text-gray-500 font-mono">Source</p>
              </div>
            </div>
          </div>

          {/* Table Chain (Destination) */}
          <div className="absolute right-0 top-1/2 -translate-y-1/2">
            <div className="flex flex-col items-center gap-2">
              {/* Chain Circle */}
              <div
                className="w-20 h-20 rounded-full flex items-center justify-center shadow-xl relative"
                style={{
                  background: `linear-gradient(135deg, ${tableColor}, ${tableColor}dd)`,
                  boxShadow: `0 0 30px ${tableColor}66`,
                }}
              >
                {/* Pulsing ring */}
                <div
                  className="absolute inset-0 rounded-full animate-ping"
                  style={{ background: `${tableColor}40`, animationDelay: '0.5s' }}
                />

                <span className="text-2xl relative z-10">♠</span>
              </div>

              {/* Label */}
              <div className="text-center">
                <p className="text-xs font-medium text-white">Table Chain</p>
                <p className="text-[10px] text-gray-500 font-mono">Destination</p>
              </div>
            </div>
          </div>

          {/* Animated Message Flow */}
          <div className="relative h-1 mx-24">
            {/* Base line */}
            <div className="absolute inset-0 bg-gradient-to-r from-gray-700 via-gray-600 to-gray-700 rounded-full" />

            {/* Animated gradient line (message in transit) */}
            <div
              className="absolute inset-0 rounded-full animate-pulse"
              style={{
                background: `linear-gradient(90deg, ${playerColor}00, ${playerColor}ff 50%, ${tableColor}ff, ${tableColor}00)`,
                backgroundSize: '200% 100%',
                animation: 'messageFlow 2s ease-in-out infinite',
              }}
            />

            {/* Arrow indicators */}
            <div className="absolute top-1/2 left-1/3 -translate-y-1/2 text-cyan-400 animate-pulse">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                <path d="M12 8l-4-4v3H4v2h4v3l4-4z" />
              </svg>
            </div>
            <div
              className="absolute top-1/2 left-2/3 -translate-y-1/2 text-yellow-400 animate-pulse"
              style={{ animationDelay: '0.5s' }}
            >
              <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                <path d="M12 8l-4-4v3H4v2h4v3l4-4z" />
              </svg>
            </div>

            {/* Message packet (traveling dot) */}
            <div
              className="absolute top-1/2 -translate-y-1/2 w-3 h-3 bg-white rounded-full shadow-lg"
              style={{
                animation: 'messageTravel 2s ease-in-out infinite',
                boxShadow: '0 0 10px rgba(255,255,255,0.8)',
              }}
            />
          </div>
        </div>

        {/* Message Details */}
        <div className="mt-8 space-y-2">
          <div className="flex items-center justify-between px-4 py-2 rounded-lg bg-white/5 border border-white/10">
            <span className="text-sm text-gray-400">Message Type</span>
            <span className="text-sm font-medium text-white font-mono">JoinTable</span>
          </div>
          <div className="flex items-center justify-between px-4 py-2 rounded-lg bg-white/5 border border-white/10">
            <span className="text-sm text-gray-400">Stake Amount</span>
            <span className="text-sm font-medium text-green-400 font-mono">{chipAmount} chips</span>
          </div>
          <div className="flex items-center justify-between px-4 py-2 rounded-lg bg-white/5 border border-white/10">
            <span className="text-sm text-gray-400">Status</span>
            <span className="text-sm font-medium text-cyan-400">Confirming...</span>
          </div>
        </div>

        {/* Loading indicator */}
        <div className="mt-6 flex items-center justify-center gap-2">
          <div className="w-2 h-2 bg-cyan-400 rounded-full animate-bounce" />
          <div className="w-2 h-2 bg-cyan-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }} />
          <div className="w-2 h-2 bg-cyan-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }} />
        </div>

        {/* Cancel Button */}
        {onCancel && (
          <div className="mt-6 flex justify-center">
            <button
              onClick={onCancel}
              className="px-6 py-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-gray-400 hover:text-white transition-all duration-300 text-sm font-medium"
            >
              Cancel
            </button>
          </div>
        )}

        {/* Screen reader status */}
        <div className="sr-only" aria-live="polite">
          Player {player} is joining the table with {chipAmount} chips. Please wait...
        </div>
      </div>

      {/* CSS animations - inject into head via style tag if needed */}
      <style>{`
        @keyframes messageFlow {
          0%, 100% {
            background-position: 0% 0%;
          }
          50% {
            background-position: 100% 0%;
          }
        }

        @keyframes messageTravel {
          0% {
            left: 0%;
            opacity: 0;
            transform: translateY(-50%) scale(0.5);
          }
          10% {
            opacity: 1;
            transform: translateY(-50%) scale(1);
          }
          90% {
            opacity: 1;
            transform: translateY(-50%) scale(1);
          }
          100% {
            left: 100%;
            opacity: 0;
            transform: translateY(-50%) scale(0.5);
          }
        }
      `}</style>
    </div>
  )
}
