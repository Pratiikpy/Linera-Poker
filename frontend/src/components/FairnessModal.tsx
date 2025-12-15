import { X, Shield, Lock, Eye, CheckCircle2 } from 'lucide-react'
import { TableState } from '../types'

interface FairnessModalProps {
  tableState: TableState | null
  onClose: () => void
}

// Convert byte array to hex string
function bytesToHex(bytes: number[] | null): string {
  if (!bytes || bytes.length === 0) return 'N/A'
  return '0x' + bytes.map(b => b.toString(16).padStart(2, '0')).join('')
}

export function FairnessModal({ tableState, onClose }: FairnessModalProps) {
  const deckSeedHex = bytesToHex(tableState?.deck_seed || null)
  const dealerSecretHex = bytesToHex(tableState?.dealer_secret || null)

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm">
      <div className="max-w-3xl w-full bg-[var(--abyss)] border border-white/10 rounded-2xl shadow-2xl overflow-hidden">
        {/* Header */}
        <div className="bg-gradient-to-r from-[var(--emerald)]/20 to-[var(--cyan)]/20 border-b border-white/10 px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-xl bg-[var(--emerald)]/20 flex items-center justify-center">
                <Shield className="w-6 h-6 text-[var(--emerald)]" />
              </div>
              <div>
                <h2 className="text-2xl font-bold text-white" style={{ fontFamily: 'Bebas Neue' }}>
                  Provably Fair Verification
                </h2>
                <p className="text-sm text-gray-400 font-mono">
                  How Linera Poker Ensures Fairness
                </p>
              </div>
            </div>
            <button
              onClick={onClose}
              className="w-10 h-10 rounded-xl bg-white/5 hover:bg-white/10 flex items-center justify-center transition-colors"
            >
              <X className="w-5 h-5 text-gray-400" />
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="px-8 py-6 space-y-6 max-h-[70vh] overflow-y-auto">
          {/* Unique Advantage */}
          <div className="bg-[var(--obsidian)] border border-[var(--emerald)]/30 rounded-xl p-6">
            <h3 className="text-lg font-bold text-[var(--gold)] mb-4 flex items-center gap-2">
              <Lock className="w-5 h-5" />
              Your Cards Are On YOUR Blockchain
            </h3>
            <p className="text-sm text-gray-300 leading-relaxed mb-3">
              Unlike traditional online poker (or other Web3 poker platforms), your hole cards are stored on
              <span className="text-[var(--emerald)] font-semibold"> your own Linera microchain</span> — not the dealer's chain.
            </p>
            <div className="flex items-start gap-2 text-sm text-gray-400">
              <CheckCircle2 className="w-5 h-5 text-[var(--emerald)] mt-0.5 flex-shrink-0" />
              <span>
                The dealer <strong className="text-[var(--crimson)]">literally cannot access</strong> your chain state.
                This is <strong>architecturally impossible</strong> without Linera's cross-chain design.
              </span>
            </div>
          </div>

          {/* How It Works */}
          <div className="space-y-4">
            <h3 className="text-lg font-bold text-white">How the Cryptographic Process Works</h3>

            {/* Step 1 */}
            <div className="flex gap-4">
              <div className="w-10 h-10 rounded-lg bg-[var(--chain-dealer)]/20 flex items-center justify-center flex-shrink-0">
                <span className="text-lg font-bold text-[var(--chain-dealer)]">1</span>
              </div>
              <div>
                <h4 className="text-sm font-semibold text-white mb-1">Deck Shuffling</h4>
                <p className="text-sm text-gray-400 leading-relaxed">
                  The dealer generates a cryptographic seed and deterministically shuffles the deck using SHA256.
                  This shuffle is <strong className="text-[var(--cyan)]">provably random</strong> and can be verified
                  by anyone who knows the seed.
                </p>
              </div>
            </div>

            {/* Step 2 */}
            <div className="flex gap-4">
              <div className="w-10 h-10 rounded-lg bg-[var(--chain-player-a)]/20 flex items-center justify-center flex-shrink-0">
                <span className="text-lg font-bold text-[var(--chain-player-a)]">2</span>
              </div>
              <div>
                <h4 className="text-sm font-semibold text-white mb-1">Card Encryption</h4>
                <p className="text-sm text-gray-400 leading-relaxed">
                  Each card is encrypted using: <code className="text-xs bg-black/30 px-2 py-1 rounded text-[var(--cyan)]">
                    SHA256(card_index + dealer_secret + nonce)
                  </code>. The dealer sends encrypted cards to your chain.
                </p>
              </div>
            </div>

            {/* Step 3 */}
            <div className="flex gap-4">
              <div className="w-10 h-10 rounded-lg bg-[var(--emerald)]/20 flex items-center justify-center flex-shrink-0">
                <span className="text-lg font-bold text-[var(--emerald)]">3</span>
              </div>
              <div>
                <h4 className="text-sm font-semibold text-white mb-1">Private Storage</h4>
                <p className="text-sm text-gray-400 leading-relaxed">
                  Your hole cards are decrypted and stored <strong className="text-[var(--emerald)]">only on your chain</strong>.
                  No central server, no dealer, no opponent — <strong>nobody</strong> can see them until you reveal at showdown.
                </p>
              </div>
            </div>

            {/* Step 4 */}
            <div className="flex gap-4">
              <div className="w-10 h-10 rounded-lg bg-[var(--gold)]/20 flex items-center justify-center flex-shrink-0">
                <span className="text-lg font-bold text-[var(--gold)]">4</span>
              </div>
              <div>
                <h4 className="text-sm font-semibold text-white mb-1">Showdown Verification</h4>
                <p className="text-sm text-gray-400 leading-relaxed">
                  At showdown, you send your cards via a cross-chain message. The dealer verifies the cards match
                  the original commitments, then determines the winner.
                </p>
              </div>
            </div>
          </div>

          {/* Cryptographic Proof */}
          {tableState?.deck_seed && tableState.deck_seed.length > 0 && (
            <div className="bg-black/30 border border-white/10 rounded-xl p-6">
              <h3 className="text-sm font-bold text-white mb-4 flex items-center gap-2">
                <Eye className="w-4 h-4 text-[var(--cyan)]" />
                Cryptographic Seeds (Game #{tableState.game_id})
              </h3>

              <div className="space-y-3">
                <div>
                  <label className="text-xs text-gray-500 font-mono block mb-1">Deck Shuffle Seed:</label>
                  <div className="bg-black/50 rounded-lg px-4 py-2 font-mono text-xs text-[var(--cyan)] break-all">
                    {deckSeedHex}
                  </div>
                </div>

                {dealerSecretHex !== 'N/A' && (
                  <div>
                    <label className="text-xs text-gray-500 font-mono block mb-1">Dealer Secret (for card commitments):</label>
                    <div className="bg-black/50 rounded-lg px-4 py-2 font-mono text-xs text-[var(--gold)] break-all">
                      {dealerSecretHex}
                    </div>
                  </div>
                )}
              </div>

              <p className="text-xs text-gray-500 mt-4">
                You can independently verify the deck shuffle using these seeds. The dealer cannot change them after the game starts.
              </p>
            </div>
          )}

          {/* Why This Matters */}
          <div className="bg-[var(--obsidian)] border border-white/10 rounded-xl p-6">
            <h3 className="text-lg font-bold text-[var(--gold)] mb-3">Why This Matters</h3>
            <ul className="space-y-2 text-sm text-gray-300">
              <li className="flex items-start gap-2">
                <CheckCircle2 className="w-4 h-4 text-[var(--emerald)] mt-0.5 flex-shrink-0" />
                <span><strong>No Central Server Can Cheat:</strong> Your cards never leave your chain.</span>
              </li>
              <li className="flex items-start gap-2">
                <CheckCircle2 className="w-4 h-4 text-[var(--emerald)] mt-0.5 flex-shrink-0" />
                <span><strong>Verifiable Shuffling:</strong> Cryptographic proof that the deck was shuffled fairly.</span>
              </li>
              <li className="flex items-start gap-2">
                <CheckCircle2 className="w-4 h-4 text-[var(--emerald)] mt-0.5 flex-shrink-0" />
                <span><strong>True Privacy:</strong> Nobody knows your cards until you choose to reveal them.</span>
              </li>
            </ul>
          </div>
        </div>

        {/* Footer */}
        <div className="border-t border-white/10 px-8 py-4 bg-[var(--obsidian)]">
          <button
            onClick={onClose}
            className="w-full py-3 rounded-xl bg-[var(--emerald)]/10 hover:bg-[var(--emerald)]/20 border border-[var(--emerald)]/30 text-[var(--emerald)] font-medium transition-all duration-300"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  )
}
