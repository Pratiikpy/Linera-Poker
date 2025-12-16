import { useEffect, useState } from 'react'
import { CheckCircle2, Circle, Loader2 } from 'lucide-react'

/**
 * ConwayConnectionLoading - Professional Conway Testnet connection animation
 *
 * Shows progressive connection steps with animations:
 * 1. @linera/client loaded
 * 2. Faucet wallet created
 * 3. Claiming chain from Conway
 * 4. Table connection
 * 5. Player chains initialized
 *
 * Features:
 * - Animated poker chip with pulse effect
 * - Gradient background (gray-900 → green-900)
 * - Professional step progression
 * - Accessibility ARIA labels
 * - Mobile responsive
 */

interface ConnectionStep {
  id: string
  label: string
  status: 'complete' | 'active' | 'pending'
}

interface ConwayConnectionLoadingProps {
  /** Current connection step (0-4) */
  currentStep?: number
  /** EVM wallet address for display */
  evmAddress?: string
}

export function ConwayConnectionLoading({
  currentStep = 0,
  evmAddress,
}: ConwayConnectionLoadingProps) {
  const [animationStep, setAnimationStep] = useState(0)

  const steps: ConnectionStep[] = [
    { id: 'client', label: '@linera/client loaded', status: 'complete' },
    { id: 'wallet', label: 'Faucet wallet created', status: currentStep >= 1 ? 'complete' : currentStep === 0 ? 'active' : 'pending' },
    { id: 'claim', label: 'Claiming chain from Conway...', status: currentStep >= 2 ? 'complete' : currentStep === 1 ? 'active' : 'pending' },
    { id: 'table', label: 'Table connection', status: currentStep >= 3 ? 'complete' : currentStep === 2 ? 'active' : 'pending' },
    { id: 'players', label: 'Player chains', status: currentStep >= 4 ? 'complete' : currentStep === 3 ? 'active' : 'pending' },
  ]

  // Animate steps progressively
  useEffect(() => {
    if (animationStep < currentStep) {
      const timer = setTimeout(() => {
        setAnimationStep(s => s + 1)
      }, 300)
      return () => clearTimeout(timer)
    }
  }, [animationStep, currentStep])

  return (
    <div
      className="min-h-screen flex items-center justify-center overflow-hidden relative"
      role="status"
      aria-live="polite"
      aria-label="Connecting to Conway Testnet"
    >
      {/* Animated gradient background */}
      <div className="absolute inset-0 bg-gradient-to-br from-gray-900 via-gray-800 to-green-900">
        <div className="absolute inset-0 opacity-30">
          <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-green-500 rounded-full blur-[128px] animate-pulse" />
          <div className="absolute bottom-1/4 right-1/4 w-96 h-96 bg-cyan-500 rounded-full blur-[128px] animate-pulse" style={{ animationDelay: '1s' }} />
        </div>
      </div>

      {/* Main content */}
      <div className="max-w-md mx-auto px-8 text-center relative z-10">
        {/* Animated Poker Chip Logo */}
        <div className="relative w-32 h-32 mx-auto mb-8">
          {/* Outer glow ring */}
          <div className="absolute inset-0 rounded-full bg-gradient-to-br from-yellow-400 to-amber-600 opacity-20 blur-xl animate-pulse" />

          {/* Spinning outer ring */}
          <div className="absolute inset-0 rounded-full border-4 border-transparent border-t-yellow-400 border-r-yellow-400 animate-spin" />

          {/* Poker chip */}
          <div className="absolute inset-4 rounded-full bg-gradient-to-br from-yellow-400 via-amber-500 to-yellow-600 flex items-center justify-center shadow-2xl">
            {/* Inner circle pattern */}
            <div className="absolute inset-2 rounded-full border-4 border-white/30" />
            <div className="absolute inset-4 rounded-full border-2 border-white/20" />

            {/* Poker suit symbols */}
            <div className="relative z-10 flex items-center justify-center">
              <span className="text-4xl text-white drop-shadow-lg animate-pulse">♠</span>
            </div>

            {/* Chip edge notches (decorative) */}
            {[...Array(8)].map((_, i) => (
              <div
                key={i}
                className="absolute w-1 h-3 bg-white/40"
                style={{
                  top: '50%',
                  left: '50%',
                  transform: `rotate(${i * 45}deg) translateY(-58px)`,
                  transformOrigin: 'center',
                }}
              />
            ))}
          </div>

          {/* Pulsing ping effect */}
          <div className="absolute inset-0 rounded-full bg-yellow-400/20 animate-ping" />
        </div>

        {/* Title */}
        <h2
          className="text-3xl font-bold mb-2 text-white"
          style={{ fontFamily: 'Bebas Neue, sans-serif', letterSpacing: '0.1em' }}
        >
          CONNECTING TO CONWAY
        </h2>

        {/* Subtitle */}
        <p className="text-gray-300 mb-8">
          Initializing Linera Testnet connection...
        </p>

        {/* EVM Wallet Display */}
        {evmAddress && (
          <div className="mb-6 px-4 py-2 rounded-lg bg-white/5 border border-white/10 backdrop-blur-sm">
            <p className="text-xs text-gray-400 mb-1">EVM Wallet</p>
            <p className="text-sm text-cyan-400 font-mono">
              {evmAddress.substring(0, 10)}...{evmAddress.substring(evmAddress.length - 8)}
            </p>
          </div>
        )}

        {/* Connection Steps */}
        <div className="space-y-3 mb-8">
          {steps.map((step, index) => (
            <div
              key={step.id}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-500 ${
                step.status === 'complete'
                  ? 'bg-green-500/10 border border-green-500/30'
                  : step.status === 'active'
                  ? 'bg-cyan-500/10 border border-cyan-500/30 shadow-lg'
                  : 'bg-white/5 border border-white/10'
              } ${
                animationStep >= index ? 'opacity-100 translate-x-0' : 'opacity-0 -translate-x-4'
              }`}
              style={{ transitionDelay: `${index * 100}ms` }}
            >
              {/* Status Icon */}
              <div className="flex-shrink-0">
                {step.status === 'complete' ? (
                  <CheckCircle2 className="w-5 h-5 text-green-400" aria-hidden="true" />
                ) : step.status === 'active' ? (
                  <Loader2 className="w-5 h-5 text-cyan-400 animate-spin" aria-hidden="true" />
                ) : (
                  <Circle className="w-5 h-5 text-gray-600" aria-hidden="true" />
                )}
              </div>

              {/* Step Label */}
              <span
                className={`text-sm font-medium transition-colors ${
                  step.status === 'complete'
                    ? 'text-green-400'
                    : step.status === 'active'
                    ? 'text-cyan-400'
                    : 'text-gray-500'
                }`}
              >
                {step.label}
              </span>

              {/* Active step pulse indicator */}
              {step.status === 'active' && (
                <div className="ml-auto flex-shrink-0">
                  <div className="flex items-center gap-1">
                    <div className="w-1.5 h-1.5 bg-cyan-400 rounded-full animate-pulse" />
                    <div className="w-1.5 h-1.5 bg-cyan-400 rounded-full animate-pulse" style={{ animationDelay: '0.2s' }} />
                    <div className="w-1.5 h-1.5 bg-cyan-400 rounded-full animate-pulse" style={{ animationDelay: '0.4s' }} />
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>

        {/* Connection Info */}
        <div className="text-xs text-gray-400 font-mono space-y-1">
          <p>Conway Testnet Faucet</p>
          <p className="text-gray-500">Powered by Dynamic Labs + @linera/client</p>
        </div>

        {/* Screen reader announcement for current step */}
        <div className="sr-only" aria-live="polite" aria-atomic="true">
          {steps.find(s => s.status === 'active')?.label || 'Connecting...'}
        </div>
      </div>
    </div>
  )
}
