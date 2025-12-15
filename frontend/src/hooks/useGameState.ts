import { useState, useEffect, useCallback, useRef } from 'react'
import {
  TableState,
  HandState,
  BetAction,
  CrossChainMessage,
  MessageType,
} from '../types'
import {
  getNetworkConfig,
  getUserFriendlyError,
  ENV,
  type ConnectionStatus,
} from '../config/network'
import { blockchainQueryService } from '../services/blockchain-query'

// Get environment variables using validated getters
const TABLE_CHAIN_ID = ENV.tableChainId()
const TABLE_APP_ID = ENV.tableAppId()
const PLAYER_A_CHAIN_ID = ENV.playerAChainId()
const PLAYER_A_HAND_APP_ID = ENV.playerAHandAppId()
const PLAYER_B_CHAIN_ID = ENV.playerBChainId()
const PLAYER_B_HAND_APP_ID = ENV.playerBHandAppId()

// GraphQL queries
const TABLE_STATE_QUERY = `
  query {
    state {
      gameId
      phase
      players {
        seat
        chainId
        stake
        currentBet
        hasFolded
        hasRevealed
      }
      pot
      currentBet
      minStake
      maxStake
      minRaise
      communityCards {
        suit
        rank
      }
      turnSeat
      winner
      deckSeed
      dealerSecret
      smallBlind
      bigBlind
      dealerButton
    }
  }
`

const HAND_STATE_QUERY = `
  query {
    state {
      tableChain
      gameId
      seat
      holeCards {
        suit
        rank
      }
      communityCards {
        suit
        rank
      }
      myTurn
      currentBet
      gameResult {
        won
        payout
      }
    }
  }
`

// Removed graphqlFetch - now using blockchainQueryService instead

// Transform snake_case to camelCase for state objects
function transformTableState(data: any): TableState | null {
  if (!data?.state) return null

  const state = data.state
  return {
    game_id: state.gameId,
    phase: state.phase,
    players: (state.players || []).map((p: any) => ({
      seat: p.seat,
      chain_id: p.chainId,
      hand_app_id: null, // Not exposed by backend
      stake: p.stake,
      current_bet: p.currentBet,
      has_folded: p.hasFolded,
      has_revealed: p.hasRevealed,
    })),
    pot: state.pot || '0',
    current_bet: state.currentBet || '0',
    min_stake: state.minStake || '10',
    max_stake: state.maxStake || '1000',
    min_raise: state.minRaise || '10',
    community_cards: state.communityCards || [],
    turn_seat: state.turnSeat,
    winner: state.winner,
    last_action_time: null, // Not exposed by backend
    deck_seed: state.deckSeed || null,
    dealer_secret: state.dealerSecret || null,
    // Standard poker blind system
    small_blind: state.smallBlind || '5.',
    big_blind: state.bigBlind || '10.',
    dealer_button: state.dealerButton || null,
  }
}

function transformHandState(data: any): HandState | null {
  if (!data?.state) return null

  const state = data.state
  return {
    table_chain_id: state.tableChain,
    table_app_id: null, // Not exposed by backend
    game_id: state.gameId,
    hole_cards: state.holeCards || [],
    my_turn: state.myTurn || false,
    current_bet: state.currentBet || '0',
    game_result: state.gameResult
      ? {
          won: state.gameResult.won,
          payout: state.gameResult.payout,
          opponent_cards: [], // Not exposed by backend
        }
      : null,
  }
}

// Mutation queries - Now call TABLE SERVICE directly (not hand service)
const JOIN_TABLE_MUTATION = `
  mutation JoinTable($playerChainId: String!, $stake: String!, $handAppId: String) {
    joinTable(playerChainId: $playerChainId, stake: $stake, handAppId: $handAppId)
  }
`

const BET_ACTION_MUTATION = `
  mutation Bet($playerChainId: String!, $action: BetActionInput!) {
    bet(playerChainId: $playerChainId, action: $action)
  }
`

const REVEAL_MUTATION = `
  mutation RevealCards($playerChainId: String!, $cards: [CardInput!]!) {
    revealCards(playerChainId: $playerChainId, cards: $cards)
  }
`

export interface UseGameStateReturn {
  // State
  tableState: TableState | null
  playerAState: HandState | null
  playerBState: HandState | null
  currentPlayer: 'A' | 'B'
  setCurrentPlayer: (player: 'A' | 'B') => void
  loading: boolean
  error: string | null
  messages: CrossChainMessage[]

  // Connection status
  connectionStatus: {
    table: ConnectionStatus
    playerA: ConnectionStatus
    playerB: ConnectionStatus
  }
  networkConfig: ReturnType<typeof getNetworkConfig>

  // Actions
  joinTable: (player: 'A' | 'B', stake: number) => Promise<void>
  placeBet: (player: 'A' | 'B', action: BetAction) => Promise<void>
  revealCards: (player: 'A' | 'B') => Promise<void>
  refreshState: () => Promise<void>
}

export function useGameState(): UseGameStateReturn {
  const [tableState, setTableState] = useState<TableState | null>(null)
  const [playerAState, setPlayerAState] = useState<HandState | null>(null)
  const [playerBState, setPlayerBState] = useState<HandState | null>(null)
  const [currentPlayer, setCurrentPlayer] = useState<'A' | 'B'>('A')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [messages, setMessages] = useState<CrossChainMessage[]>([])

  // Connection status for each endpoint
  const [connectionStatus, setConnectionStatus] = useState<{
    table: ConnectionStatus
    playerA: ConnectionStatus
    playerB: ConnectionStatus
  }>({
    table: 'connecting',
    playerA: 'connecting',
    playerB: 'connecting',
  })

  // Get network configuration
  const networkConfig = getNetworkConfig()

  const messageIdCounter = useRef(0)

  // Add a cross-chain message to the log
  const addMessage = useCallback(
    (type: MessageType, from: string, to: string, data?: unknown) => {
      const message: CrossChainMessage = {
        id: `msg-${messageIdCounter.current++}`,
        type,
        from,
        to,
        timestamp: Date.now(),
        data,
      }
      setMessages((prev) => [...prev.slice(-19), message]) // Keep last 20
    },
    []
  )

  // Fetch all state using blockchain query service
  const fetchState = useCallback(async () => {
    // Check if blockchain query service is initialized
    if (!blockchainQueryService.isInitialized()) {
      console.log('⚠️ [Game State] Blockchain query service not initialized yet')
      return
    }

    try {
      // Fetch table state using blockchain query service
      if (TABLE_CHAIN_ID && TABLE_APP_ID) {
        try {
          setConnectionStatus(prev => ({ ...prev, table: 'connecting' }))

          // Parse GraphQL query to object format for service
          const queryObj = {
            query: TABLE_STATE_QUERY
          }

          const tableData = await blockchainQueryService.queryTableState(
            TABLE_CHAIN_ID,
            TABLE_APP_ID,
            queryObj
          )

          setTableState(transformTableState(tableData))
          setConnectionStatus(prev => ({ ...prev, table: 'connected' }))
        } catch (err) {
          console.error('Failed to fetch table state:', err)
          setConnectionStatus(prev => ({ ...prev, table: 'error' }))
          // Don't throw - allow other fetches to continue
        }
      }

      // Fetch player A state using blockchain query service
      if (PLAYER_A_CHAIN_ID && PLAYER_A_HAND_APP_ID) {
        try {
          setConnectionStatus(prev => ({ ...prev, playerA: 'connecting' }))

          const queryObj = {
            query: HAND_STATE_QUERY
          }

          const playerAData = await blockchainQueryService.queryPlayerAState(
            PLAYER_A_CHAIN_ID,
            PLAYER_A_HAND_APP_ID,
            queryObj
          )

          setPlayerAState(transformHandState(playerAData))
          setConnectionStatus(prev => ({ ...prev, playerA: 'connected' }))
        } catch (err) {
          console.error('Failed to fetch player A state:', err)
          setConnectionStatus(prev => ({ ...prev, playerA: 'error' }))
          // Don't throw - allow other fetches to continue
        }
      }

      // Fetch player B state using blockchain query service
      if (PLAYER_B_CHAIN_ID && PLAYER_B_HAND_APP_ID) {
        try {
          setConnectionStatus(prev => ({ ...prev, playerB: 'connecting' }))

          const queryObj = {
            query: HAND_STATE_QUERY
          }

          const playerBData = await blockchainQueryService.queryPlayerBState(
            PLAYER_B_CHAIN_ID,
            PLAYER_B_HAND_APP_ID,
            queryObj
          )

          setPlayerBState(transformHandState(playerBData))
          setConnectionStatus(prev => ({ ...prev, playerB: 'connected' }))
        } catch (err) {
          console.error('Failed to fetch player B state:', err)
          setConnectionStatus(prev => ({ ...prev, playerB: 'error' }))
          // Don't throw - allow other fetches to continue
        }
      }

      setError(null)
    } catch (err) {
      console.error('Failed to fetch state:', err)
      const friendlyError = getUserFriendlyError(err, 'fetching game state')
      setError(friendlyError)
    }
  }, [])

  // Initial fetch and polling
  useEffect(() => {
    fetchState()

    // Poll every 2 seconds
    const interval = setInterval(fetchState, 2000)

    return () => clearInterval(interval)
  }, [fetchState])

  // Join table action - uses blockchain query service
  const joinTable = useCallback(
    async (player: 'A' | 'B', stake: number) => {
      setLoading(true)
      setError(null)

      try {
        const playerChainId = player === 'A' ? PLAYER_A_CHAIN_ID : PLAYER_B_CHAIN_ID
        const handAppId = player === 'A' ? PLAYER_A_HAND_APP_ID : PLAYER_B_HAND_APP_ID

        if (!playerChainId) {
          throw new Error(`Player ${player} chain not configured. Please run deployment script.`)
        }

        if (!TABLE_CHAIN_ID || !TABLE_APP_ID) {
          throw new Error('Table not configured. Please run deployment script.')
        }

        // Use blockchain query service for mutation
        const mutationObj = {
          query: JOIN_TABLE_MUTATION,
          variables: {
            playerChainId,
            stake: stake.toString(),
            handAppId: handAppId || null,
          }
        }

        await blockchainQueryService.mutateTable(
          TABLE_CHAIN_ID,
          TABLE_APP_ID,
          mutationObj
        )

        // Log the action
        addMessage(
          'JoinTable',
          `Player ${player}`,
          'Table',
          { stake }
        )

        // Refresh state after action
        await fetchState()
      } catch (err) {
        console.error('Join table failed:', err)
        const friendlyError = getUserFriendlyError(err, 'joining table')
        setError(friendlyError)
      } finally {
        setLoading(false)
      }
    },
    [fetchState, addMessage]
  )

  // Bet action - uses blockchain query service
  const bet = useCallback(
    async (player: 'A' | 'B', action: BetAction) => {
      setLoading(true)
      setError(null)

      try {
        const playerChainId = player === 'A' ? PLAYER_A_CHAIN_ID : PLAYER_B_CHAIN_ID

        if (!playerChainId) {
          throw new Error(`Player ${player} chain not configured. Please run deployment script.`)
        }

        if (!TABLE_CHAIN_ID || !TABLE_APP_ID) {
          throw new Error('Table not configured. Please run deployment script.')
        }

        // Convert BetAction to GraphQL input format
        let actionInput: Record<string, unknown>
        if ('Check' in action) {
          actionInput = { actionType: 'CHECK' }
        } else if ('Call' in action) {
          actionInput = { actionType: 'CALL' }
        } else if ('Raise' in action) {
          actionInput = { actionType: 'RAISE', amount: action.Raise }
        } else if ('AllIn' in action) {
          actionInput = { actionType: 'ALL_IN' }
        } else if ('Fold' in action) {
          actionInput = { actionType: 'FOLD' }
        } else {
          throw new Error('Invalid bet action')
        }

        // Use blockchain query service for mutation
        const mutationObj = {
          query: BET_ACTION_MUTATION,
          variables: {
            playerChainId,
            action: actionInput,
          }
        }

        await blockchainQueryService.mutateTable(
          TABLE_CHAIN_ID,
          TABLE_APP_ID,
          mutationObj
        )

        // Log the action
        addMessage(
          'BetAction',
          `Player ${player}`,
          'Table',
          { action: Object.keys(action)[0] }
        )

        // Refresh state after action
        await fetchState()
      } catch (err) {
        console.error('Bet failed:', err)
        const friendlyError = getUserFriendlyError(err, 'placing bet')
        setError(friendlyError)
      } finally {
        setLoading(false)
      }
    },
    [fetchState, addMessage]
  )

  // Reveal cards action - uses blockchain query service
  const reveal = useCallback(
    async (player: 'A' | 'B') => {
      setLoading(true)
      setError(null)

      try {
        const playerChainId = player === 'A' ? PLAYER_A_CHAIN_ID : PLAYER_B_CHAIN_ID
        const handState = player === 'A' ? playerAState : playerBState

        if (!playerChainId) {
          throw new Error(`Player ${player} chain not configured. Please run deployment script.`)
        }

        if (!TABLE_CHAIN_ID || !TABLE_APP_ID) {
          throw new Error('Table not configured. Please run deployment script.')
        }

        // Get player's cards from hand state (for now, send empty cards for testing)
        const cards = handState?.hole_cards?.map(c => ({
          suit: c.suit || 'Spades',
          rank: c.rank || 'Two',
        })) || []

        // Use blockchain query service for mutation
        const mutationObj = {
          query: REVEAL_MUTATION,
          variables: {
            playerChainId,
            cards,
          }
        }

        await blockchainQueryService.mutateTable(
          TABLE_CHAIN_ID,
          TABLE_APP_ID,
          mutationObj
        )

        // Log the action
        addMessage('RevealCards', `Player ${player}`, 'Table')

        // Refresh state after action
        await fetchState()
      } catch (err) {
        console.error('Reveal failed:', err)
        const friendlyError = getUserFriendlyError(err, 'revealing cards')
        setError(friendlyError)
      } finally {
        setLoading(false)
      }
    },
    [fetchState, addMessage, playerAState, playerBState]
  )

  return {
    tableState,
    playerAState,
    playerBState,
    currentPlayer,
    setCurrentPlayer,
    loading,
    error,
    messages,
    connectionStatus,
    networkConfig,
    joinTable,
    placeBet: bet,
    revealCards: reveal,
    refreshState: fetchState,
  }
}
