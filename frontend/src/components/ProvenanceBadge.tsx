import { Shield } from 'lucide-react'
import { TableState } from '../types'

interface ProvenanceBadgeProps {
  tableState: TableState | null
  onShowDetails: () => void
}

export function ProvenanceBadge({ tableState, onShowDetails }: ProvenanceBadgeProps) {
  if (!tableState || !tableState.game_id) {
    return null
  }

  return (
    <button
      onClick={onShowDetails}
      className="inline-flex items-center gap-2 px-4 py-2 rounded-xl bg-[var(--emerald)]/10 border border-[var(--emerald)]/30 hover:border-[var(--emerald)] transition-all duration-300 group"
      title="Click to verify game fairness"
    >
      <Shield className="w-4 h-4 text-[var(--emerald)] group-hover:scale-110 transition-transform" />
      <span className="text-sm font-medium text-[var(--emerald)]">Provably Fair</span>
      <span className="text-xs text-gray-500 font-mono">
        Game #{tableState.game_id}
      </span>
    </button>
  )
}
