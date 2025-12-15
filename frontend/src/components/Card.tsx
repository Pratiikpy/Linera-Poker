interface CardProps {
  card: {
    suit: string
    rank: string
  }
  hidden?: boolean
  dealt?: boolean
  index?: number
  winning?: boolean
  isActivePlayer?: boolean
  shimmer?: boolean
}

const SUIT_SYMBOLS: Record<string, { symbol: string; isRed: boolean }> = {
  Hearts: { symbol: '♥', isRed: true },
  Diamonds: { symbol: '♦', isRed: true },
  Clubs: { symbol: '♣', isRed: false },
  Spades: { symbol: '♠', isRed: false },
}

const RANK_DISPLAY: Record<string, string> = {
  Two: '2',
  Three: '3',
  Four: '4',
  Five: '5',
  Six: '6',
  Seven: '7',
  Eight: '8',
  Nine: '9',
  Ten: '10',
  Jack: 'J',
  Queen: 'Q',
  King: 'K',
  Ace: 'A',
}

export function Card({
  card,
  hidden = false,
  dealt = false,
  index = 0,
  winning = false,
  isActivePlayer = false,
  shimmer = false
}: CardProps) {
  const dealDelay = index * 0.15

  if (hidden) {
    return (
      <div
        className={`card card-back card-mystery ${dealt ? 'card-dealt' : ''}`}
        style={{ animationDelay: `${dealDelay}s` }}
      />
    )
  }

  const suitInfo = SUIT_SYMBOLS[card.suit] || { symbol: card.suit, isRed: false }
  const rank = RANK_DISPLAY[card.rank] || card.rank

  const cardClasses = [
    'card',
    'card-front',
    suitInfo.isRed ? 'card-red' : 'card-black',
    dealt && !shimmer ? 'card-dealt' : '',
    shimmer ? 'card-shimmer' : '',
    winning ? 'card-winning' : '',
    isActivePlayer ? 'card-active-player' : ''
  ].filter(Boolean).join(' ')

  return (
    <div
      className={cardClasses}
      style={{ animationDelay: `${dealDelay}s` }}
    >
      {/* Top-left corner */}
      <div className="absolute top-2 left-2 flex flex-col items-center leading-none">
        <span className="text-lg font-bold">{rank}</span>
        <span className="text-xl">{suitInfo.symbol}</span>
      </div>

      {/* Center suit - Larger and more dramatic */}
      <div className="flex flex-col items-center">
        <span className="text-4xl drop-shadow-lg">{suitInfo.symbol}</span>
      </div>

      {/* Bottom-right corner (rotated) */}
      <div className="absolute bottom-2 right-2 flex flex-col items-center leading-none rotate-180">
        <span className="text-lg font-bold">{rank}</span>
        <span className="text-xl">{suitInfo.symbol}</span>
      </div>
    </div>
  )
}

export function CardBack({
  dealt = false,
  index = 0,
  mystery = false
}: {
  dealt?: boolean
  index?: number
  mystery?: boolean
}) {
  const dealDelay = index * 0.15

  return (
    <div
      className={`card card-back ${dealt ? 'card-dealt' : ''} ${mystery ? 'card-mystery' : ''}`}
      style={{ animationDelay: `${dealDelay}s` }}
    />
  )
}

export function CardPlaceholder() {
  return (
    <div className="card card-placeholder">
      <span className="text-2xl opacity-30">?</span>
    </div>
  )
}

export function CardSkeleton({ index = 0 }: { index?: number }) {
  const dealDelay = index * 0.15

  return (
    <div
      className="card card-skeleton"
      style={{ animationDelay: `${dealDelay}s` }}
    >
      <div className="absolute top-2 left-2 w-4 h-6 bg-gray-600 rounded opacity-20" />
      <div className="w-8 h-8 bg-gray-600 rounded-full opacity-20" />
      <div className="absolute bottom-2 right-2 w-4 h-6 bg-gray-600 rounded opacity-20" />
    </div>
  )
}
