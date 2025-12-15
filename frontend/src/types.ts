// Card types matching Rust contract types
export interface Card {
  suit: 'Hearts' | 'Diamonds' | 'Clubs' | 'Spades'
  rank: 'Two' | 'Three' | 'Four' | 'Five' | 'Six' | 'Seven' | 'Eight' | 'Nine' | 'Ten' | 'Jack' | 'Queen' | 'King' | 'Ace'
}

export interface EncryptedCard {
  encrypted_data: string
  commitment: string
}

export interface CardReveal {
  card: Card
  commitment: string
  salt: string
}

// Game phases matching Rust GamePhase enum
export type GamePhase =
  | 'WaitingForPlayers'
  | 'Dealing'
  | 'PreFlop'
  | 'Flop'
  | 'Turn'
  | 'River'
  | 'Showdown'
  | 'Settlement'
  | 'Finished'

// Player seat identifiers
export type Seat = 'Player1' | 'Player2'

// Betting actions matching Rust BetAction enum
export type BetAction =
  | { Check: null }
  | { Call: null }
  | { Raise: string }
  | { AllIn: null }
  | { Fold: null }

// Player info at the table
export interface PlayerInfo {
  seat: Seat
  chain_id: string
  hand_app_id: string | null
  stake: string
  current_bet: string
  has_folded: boolean
  has_revealed: boolean
}

// Table state from dealer chain
export interface TableState {
  game_id: number | null
  phase: GamePhase
  players: PlayerInfo[]
  pot: string
  current_bet: string
  min_stake: string
  max_stake: string
  min_raise: string
  community_cards: Card[]
  turn_seat: Seat | null
  winner: Seat | null
  last_action_time: number | null
  deck_seed: number[] | null  // For provable fairness verification
  dealer_secret: number[] | null  // For card commitment verification
  // Standard poker blind system
  small_blind: string
  big_blind: string
  dealer_button: Seat | null  // Who has the dealer button (SB position)
}

// Game result sent to players
export interface GameResult {
  won: boolean
  payout: string
  opponent_cards?: Card[]
}

// Hand state from player chain
export interface HandState {
  table_chain_id: string | null
  table_app_id: string | null
  game_id: number | null
  hole_cards: Card[] | null
  my_turn: boolean
  current_bet: string
  game_result: GameResult | null
}

// Token state from player chain
export interface TokenState {
  balance: string
  owner: string
  staked_amount: string
  staked_game_id: number | null
}

// GraphQL query responses
export interface TableQueryResponse {
  tableState: TableState
}

export interface HandQueryResponse {
  handState: HandState
}

export interface TokenQueryResponse {
  tokenState: TokenState
}

// Operations for mutations
export interface JoinTableArgs {
  stake: number
  tableChainId: string
  tableAppId: string
}

export interface BetArgs {
  action: BetAction
}

export interface RevealArgs {
  // No args needed - contract handles it
}

// Cross-chain message types (for visualization)
export type MessageType =
  | 'JoinTable'
  | 'DealCards'
  | 'BetAction'
  | 'YourTurn'
  | 'RevealCards'
  | 'GameResult'
  | 'Settlement'

export interface CrossChainMessage {
  id: string
  type: MessageType
  from: string
  to: string
  timestamp: number
  data?: unknown
}

// UI state
export interface GameUIState {
  loading: boolean
  error: string | null
  currentPlayer: 'A' | 'B'
  messages: CrossChainMessage[]
}
