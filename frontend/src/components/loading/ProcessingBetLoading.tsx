import { Loader2 } from 'lucide-react'

/**
 * ProcessingBetLoading - Inline loading indicator for betting actions
 *
 * Small, non-intrusive loading spinner shown during bet processing.
 * Context-aware messaging based on the action type.
 *
 * Features:
 * - Inline, compact design
 * - Context-aware messages (Call, Raise, Fold, Check)
 * - Professional spinner animation
 * - Accessibility compliant
 * - Minimal screen real estate
 */

type BetAction = 'call' | 'raise' | 'fold' | 'check' | 'reveal' | 'deal'

interface ProcessingBetLoadingProps {
  /** Type of action being processed */
  action?: BetAction
  /** Custom message override */
  message?: string
  /** Size variant */
  size?: 'sm' | 'md' | 'lg'
  /** Show as inline (default) or block */
  inline?: boolean
}

const ACTION_MESSAGES: Record<BetAction, string> = {
  call: 'Processing Call...',
  raise: 'Processing Raise...',
  fold: 'Processing Fold...',
  check: 'Processing Check...',
  reveal: 'Revealing Cards...',
  deal: 'Dealing Cards...',
}

const SIZE_CLASSES = {
  sm: {
    spinner: 'w-4 h-4',
    text: 'text-xs',
    padding: 'px-3 py-1.5',
  },
  md: {
    spinner: 'w-5 h-5',
    text: 'text-sm',
    padding: 'px-4 py-2',
  },
  lg: {
    spinner: 'w-6 h-6',
    text: 'text-base',
    padding: 'px-5 py-3',
  },
}

export function ProcessingBetLoading({
  action = 'call',
  message,
  size = 'md',
  inline = true,
}: ProcessingBetLoadingProps) {
  const displayMessage = message || ACTION_MESSAGES[action] || 'Processing...'
  const sizeConfig = SIZE_CLASSES[size]

  return (
    <div
      className={`
        flex items-center gap-2
        ${sizeConfig.padding}
        rounded-lg
        bg-cyan-500/10
        border border-cyan-500/30
        ${inline ? 'inline-flex' : 'flex'}
      `}
      role="status"
      aria-live="polite"
      aria-label={displayMessage}
    >
      {/* Spinner */}
      <Loader2
        className={`${sizeConfig.spinner} text-cyan-400 animate-spin`}
        aria-hidden="true"
      />

      {/* Message */}
      <span className={`${sizeConfig.text} font-medium text-cyan-400`}>
        {displayMessage}
      </span>

      {/* Animated dots */}
      <div className="flex items-center gap-0.5 ml-1">
        <div
          className="w-1 h-1 bg-cyan-400 rounded-full animate-pulse"
          style={{ animationDelay: '0s' }}
        />
        <div
          className="w-1 h-1 bg-cyan-400 rounded-full animate-pulse"
          style={{ animationDelay: '0.2s' }}
        />
        <div
          className="w-1 h-1 bg-cyan-400 rounded-full animate-pulse"
          style={{ animationDelay: '0.4s' }}
        />
      </div>

      {/* Screen reader only text */}
      <span className="sr-only">{displayMessage}</span>
    </div>
  )
}

/**
 * ProcessingBetOverlay - Full overlay variant for blocking operations
 *
 * Use this when you need to prevent user interaction during processing.
 * Shows a semi-transparent overlay with centered loading indicator.
 */
interface ProcessingBetOverlayProps {
  action?: BetAction
  message?: string
}

export function ProcessingBetOverlay({
  action = 'call',
  message,
}: ProcessingBetOverlayProps) {
  const displayMessage = message || ACTION_MESSAGES[action] || 'Processing...'

  return (
    <div
      className="fixed inset-0 z-40 flex items-center justify-center bg-black/40 backdrop-blur-sm"
      role="status"
      aria-live="polite"
      aria-label={displayMessage}
    >
      <div className="bg-gray-900/90 backdrop-blur-xl rounded-2xl border border-white/10 p-8 shadow-2xl max-w-sm">
        {/* Large spinner */}
        <div className="flex justify-center mb-6">
          <div className="relative">
            {/* Outer glow ring */}
            <div className="absolute inset-0 rounded-full bg-cyan-400 opacity-20 blur-xl animate-pulse" />

            {/* Spinning ring */}
            <div className="relative w-20 h-20 rounded-full border-4 border-gray-700">
              <div className="absolute inset-0 rounded-full border-4 border-transparent border-t-cyan-400 border-r-cyan-400 animate-spin" />
            </div>

            {/* Center icon */}
            <div className="absolute inset-0 flex items-center justify-center">
              <Loader2 className="w-8 h-8 text-cyan-400 animate-spin" />
            </div>
          </div>
        </div>

        {/* Message */}
        <p className="text-center text-lg font-medium text-white mb-2">
          {displayMessage}
        </p>

        {/* Submessage */}
        <p className="text-center text-sm text-gray-400">
          Submitting cross-chain transaction...
        </p>

        {/* Loading dots */}
        <div className="flex items-center justify-center gap-2 mt-6">
          <div className="w-2 h-2 bg-cyan-400 rounded-full animate-bounce" />
          <div className="w-2 h-2 bg-cyan-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }} />
          <div className="w-2 h-2 bg-cyan-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }} />
        </div>
      </div>
    </div>
  )
}
