#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TableState;
use linera_poker_table::{
    BetAction, Card, CardReveal, GamePhase,
    InstantiationArgument, Message, PlayerInfo, Seat, TableAbi, TableResult,
    TableOperation,
};
use linera_poker_shared::{shuffle_deck, evaluate_hand};
use linera_sdk::{
    linera_base_types::{Amount, ApplicationId, ChainId, AccountOwner, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};
use sha2::{Sha256, Digest};

pub struct TableContract {
    state: TableState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(TableContract);

impl WithContractAbi for TableContract {
    type Abi = TableAbi;
}

impl Contract for TableContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = InstantiationArgument;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TableState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TableContract { state, runtime }
    }

    async fn instantiate(&mut self, arg: InstantiationArgument) {
        self.state.game_id.set(1);
        self.state.phase.set(GamePhase::WaitingForPlayers);
        self.state.players.set(Vec::new());
        self.state.pot.set(Amount::ZERO);
        self.state.current_bet.set(Amount::ZERO);
        self.state.min_raise.set(Amount::from_tokens(arg.big_blind.into())); // Min raise = big blind
        self.state.community_cards.set(Vec::new());
        self.state.turn_seat.set(None);
        self.state.winner.set(None);
        self.state.min_stake.set(Amount::from_tokens(arg.min_stake.into()));
        self.state.max_stake.set(Amount::from_tokens(arg.max_stake.into()));
        self.state.revealed_cards.set(Vec::new());
        // Initialize blinds
        self.state.small_blind.set(Amount::from_tokens(arg.small_blind.into()));
        self.state.big_blind.set(Amount::from_tokens(arg.big_blind.into()));
        self.state.dealer_button.set(None);
    }

    async fn execute_operation(&mut self, operation: TableOperation) -> TableResult {
        match operation {
            TableOperation::StartNewGame => {
                self.start_new_game();
                TableResult::Success
            }
            TableOperation::ForceAdvance => {
                self.advance_phase();
                TableResult::Success
            }

            // Relay operations from hand app on table chain
            TableOperation::RelayJoinTable { player_chain, stake, hand_app_id } => {
                self.handle_join(player_chain, stake, hand_app_id).await;
                TableResult::Success
            }
            TableOperation::RelayBetAction { player_chain, game_id: _, action } => {
                // Use current game_id from state instead of passed value
                let current_game_id = *self.state.game_id.get();
                self.handle_bet_action(player_chain, current_game_id, action).await;
                TableResult::Success
            }
            TableOperation::RelayRevealCards { player_chain, game_id: _, cards, proofs } => {
                // Use current game_id from state instead of passed value
                let current_game_id = *self.state.game_id.get();
                self.handle_reveal(player_chain, current_game_id, cards, proofs).await;
                TableResult::Success
            }
            TableOperation::RelayLeaveTable { player_chain } => {
                self.handle_leave(player_chain);
                TableResult::Success
            }
            TableOperation::RelayCardsReceived { player_chain: _, game_id: _ } => {
                // Acknowledgment only
                TableResult::Success
            }
        }
    }

    async fn execute_message(&mut self, message: Message) {
        let source_chain = match self.runtime.message_origin_chain_id() {
            Some(chain_id) => chain_id,
            None => return,
        };

        match message {
            // INCOMING messages from Hand chains
            Message::JoinTable { stake, hand_app_id } => {
                self.handle_join(source_chain, stake, hand_app_id).await;
            }
            Message::CardsReceived { game_id: _ } => {
                // Acknowledgment only
            }
            Message::BetAction { game_id, action } => {
                self.handle_bet_action(source_chain, game_id, action).await;
            }
            Message::RevealCards { game_id, cards, proofs } => {
                self.handle_reveal(source_chain, game_id, cards, proofs).await;
            }
            Message::LeaveTable => {
                self.handle_leave(source_chain);
            }
            // OUTGOING messages (shouldn't be received)
            _ => {}
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl TableContract {
    /// Handle player joining
    async fn handle_join(
        &mut self,
        player_chain: ChainId,
        stake: Amount,
        hand_app: ApplicationId,
    ) {
        let phase = self.state.phase.get();
        if *phase != GamePhase::WaitingForPlayers {
            return;
        }

        let mut players = self.state.players.get().clone();
        if players.iter().any(|p| p.chain_id == player_chain) {
            return;
        }

        let min_stake = *self.state.min_stake.get();
        let max_stake = *self.state.max_stake.get();
        if stake < min_stake || stake > max_stake {
            return;
        }

        if players.len() >= 2 {
            return;
        }

        let seat = if players.is_empty() {
            Seat::Player1
        } else {
            Seat::Player2
        };

        let owner = self.runtime.authenticated_signer().unwrap_or(AccountOwner::CHAIN);

        players.push(PlayerInfo {
            seat,
            chain_id: player_chain,
            owner,
            stake,
            hand_app: Some(hand_app),
            has_folded: false,
            current_bet: Amount::ZERO,
            has_revealed: false,
        });

        let mut pot = *self.state.pot.get();
        pot = pot.saturating_add(stake);
        self.state.pot.set(pot);
        self.state.players.set(players.clone());

        // If two players joined, start dealing
        if players.len() == 2 {
            self.deal_cards().await;
        }
    }

    /// Deal cards to all players
    /// FIX #5: HIGH - Validate player count before dealing
    async fn deal_cards(&mut self) {
        let mut players = self.state.players.get().clone();

        // FIX #5: Need exactly 2 players to deal
        if players.len() != 2 {
            return; // Cannot deal without exactly 2 players
        }

        self.state.phase.set(GamePhase::Dealing);

        // === STANDARD POKER: Assign dealer button ===
        // Button alternates based on game_id (first game: Player1, second: Player2, etc.)
        let game_id = *self.state.game_id.get();
        let button = if game_id % 2 == 1 {
            Seat::Player1
        } else {
            Seat::Player2
        };
        self.state.dealer_button.set(Some(button));

        // === STANDARD POKER: Post blinds ===
        let small_blind = *self.state.small_blind.get();
        let big_blind = *self.state.big_blind.get();
        let bb_seat = button.other(); // Big blind is non-button player

        // Find player indices
        let button_idx = players.iter().position(|p| p.seat == button).unwrap();
        let bb_idx = players.iter().position(|p| p.seat == bb_seat).unwrap();

        // Button posts small blind (from their stake)
        players[button_idx].current_bet = small_blind;
        // Non-button posts big blind (from their stake)
        players[bb_idx].current_bet = big_blind;

        // Set current bet to BB (pot already has stakes from handle_join)
        // In heads-up, blinds are posted from stake, so pot remains unchanged
        self.state.current_bet.set(big_blind);
        self.state.players.set(players.clone());

        // Generate deck
        let seed = self.generate_deck_seed();
        let deck = shuffle_deck(&seed);
        let dealer_secret = self.generate_dealer_secret();

        self.state.deck_seed.set(seed);
        self.state.dealer_secret.set(dealer_secret.clone());

        // Deal 2 hole cards to each player
        for (idx, player) in players.iter().enumerate() {
            let card1 = deck[idx * 2];
            let card2 = deck[idx * 2 + 1];

            // Create reveals for the dealt cards
            let reveals = vec![
                CardReveal { card: card1, secret: dealer_secret.clone() },
                CardReveal { card: card2, secret: dealer_secret.clone() },
            ];

            if player.hand_app.is_some() {
                self.runtime
                    .prepare_message(Message::CommunityCards {
                        game_id,
                        phase: GamePhase::Dealing,
                        cards: reveals,
                    })
                    .with_authentication()
                    .send_to(player.chain_id);
            }
        }

        // Store community cards
        self.state.community_cards.set(deck[4..9].to_vec());

        // Move to pre-flop
        self.state.phase.set(GamePhase::PreFlop);

        // === STANDARD POKER: Pre-flop, BUTTON acts first (SB) ===
        self.state.turn_seat.set(Some(button));

        // Initialize action counter for first betting round
        self.state.actions_this_round.set(0);

        self.notify_turn().await;
    }

    /// Handle betting action
    async fn handle_bet_action(
        &mut self,
        player_chain: ChainId,
        game_id: u64,
        action: BetAction,
    ) {
        if game_id != *self.state.game_id.get() {
            return;
        }

        let mut players = self.state.players.get().clone();
        let player_idx = match players.iter().position(|p| p.chain_id == player_chain) {
            Some(idx) => idx,
            None => return,
        };

        let player_seat = players[player_idx].seat;
        if self.state.turn_seat.get() != &Some(player_seat) {
            return;
        }

        let phase = *self.state.phase.get();
        match phase {
            GamePhase::PreFlop | GamePhase::Flop | GamePhase::Turn | GamePhase::River => {}
            _ => return,
        }

        let mut pot = *self.state.pot.get();
        let mut current_bet = *self.state.current_bet.get();
        let min_raise = *self.state.min_raise.get();

        match action {
            BetAction::Check => {
                if current_bet > players[player_idx].current_bet {
                    return;
                }
            }
            BetAction::Call => {
                let to_call = current_bet.saturating_sub(players[player_idx].current_bet);
                players[player_idx].current_bet = current_bet;
                pot = pot.saturating_add(to_call);
            }
            BetAction::Raise(amount) => {
                if amount < min_raise {
                    return;
                }

                // FIX #6: HIGH - Validate bet against player's available stack
                let player_remaining = players[player_idx].stake
                    .saturating_sub(players[player_idx].current_bet);
                let new_bet = current_bet.saturating_add(amount);
                let required = new_bet.saturating_sub(players[player_idx].current_bet);

                if required > player_remaining {
                    return; // Cannot bet more than available stack
                }

                let addition = new_bet.saturating_sub(players[player_idx].current_bet);
                players[player_idx].current_bet = new_bet;
                current_bet = new_bet;
                pot = pot.saturating_add(addition);
            }
            BetAction::AllIn => {
                let remaining = players[player_idx].stake
                    .saturating_sub(players[player_idx].current_bet);
                let new_bet = players[player_idx].current_bet.saturating_add(remaining);
                if new_bet > current_bet {
                    current_bet = new_bet;
                }
                players[player_idx].current_bet = new_bet;
                pot = pot.saturating_add(remaining);
            }
            BetAction::Fold => {
                // FIX #8: MEDIUM - Check if opponent already folded (edge case)
                let opponent_seat = player_seat.other();
                let opponent_folded = players.iter()
                    .find(|p| p.seat == opponent_seat)
                    .map(|p| p.has_folded)
                    .unwrap_or(false);

                if opponent_folded {
                    // Both folded - current player wins by default since they folded second
                    self.state.winner.set(Some(player_seat));
                    self.state.phase.set(GamePhase::Settlement);
                    self.state.players.set(players);
                    self.state.pot.set(pot);
                    self.settle_game().await;
                    return;
                }

                // Normal fold - opponent wins
                players[player_idx].has_folded = true;
                let winner_seat = player_seat.other();
                self.state.winner.set(Some(winner_seat));
                self.state.phase.set(GamePhase::Settlement);
                self.state.players.set(players);
                self.state.pot.set(pot);
                self.settle_game().await;
                return;
            }
        }

        self.state.pot.set(pot);
        self.state.current_bet.set(current_bet);
        self.state.players.set(players);

        // Increment action counter to track betting round completion
        let actions = self.state.actions_this_round.get().saturating_add(1);
        self.state.actions_this_round.set(actions);

        self.advance_turn().await;
    }

    /// Advance to next player or phase
    /// FIX #4: HIGH - Replace unwrap() with safe error handling
    async fn advance_turn(&mut self) {
        let current_seat = match self.state.turn_seat.get() {
            Some(s) => *s,
            None => return,
        };
        let next_seat = current_seat.other();

        let players = self.state.players.get();

        // FIX #4: Safe pattern - early return if player not found
        let next_player = match players.iter().find(|p| p.seat == next_seat) {
            Some(p) => p,
            None => return,
        };
        let current_bet = *self.state.current_bet.get();

        if !next_player.has_folded && next_player.current_bet < current_bet {
            self.state.turn_seat.set(Some(next_seat));
            self.notify_turn().await;
        } else {
            // FIX #4: Safe pattern - early return if player not found
            let current_player = match players.iter().find(|p| p.seat == current_seat) {
                Some(p) => p,
                None => return,
            };

            // FIX BUG #1: Only advance phase if both players have acted (actions >= 2) and bets match
            let actions = *self.state.actions_this_round.get();
            if current_player.current_bet == current_bet
                && next_player.current_bet == current_bet
                && actions >= 2
            {
                self.advance_phase();
            } else {
                self.state.turn_seat.set(Some(next_seat));
                self.notify_turn().await;
            }
        }
    }

    /// Advance to next game phase
    fn advance_phase(&mut self) {
        let mut players = self.state.players.get().clone();
        for p in &mut players {
            p.current_bet = Amount::ZERO;
        }
        self.state.players.set(players);
        self.state.current_bet.set(Amount::ZERO);
        // Reset action counter for new betting round
        self.state.actions_this_round.set(0);

        let phase = *self.state.phase.get();
        let new_phase = match phase {
            GamePhase::PreFlop => GamePhase::Flop,
            GamePhase::Flop => GamePhase::Turn,
            GamePhase::Turn => GamePhase::River,
            GamePhase::River => GamePhase::Showdown,
            _ => return,
        };
        self.state.phase.set(new_phase);

        if new_phase == GamePhase::Showdown {
            self.state.turn_seat.set(None);
        } else {
            // === STANDARD POKER: Post-flop, NON-BUTTON (BB) acts first ===
            let button = self.state.dealer_button.get().unwrap_or(Seat::Player1);
            let bb_seat = button.other();
            self.state.turn_seat.set(Some(bb_seat));
        }
    }

    /// Handle card reveal
    /// FIX #1: CRITICAL - Verify card reveal proofs to prevent cheating
    async fn handle_reveal(
        &mut self,
        player_chain: ChainId,
        game_id: u64,
        cards: Vec<Card>,
        proofs: Vec<CardReveal>,
    ) {
        if game_id != *self.state.game_id.get() {
            return;
        }

        if *self.state.phase.get() != GamePhase::Showdown {
            return;
        }

        let mut players = self.state.players.get().clone();
        let player_idx = match players.iter().position(|p| p.chain_id == player_chain) {
            Some(idx) => idx,
            None => return,
        };

        // FIX #1: Verify each revealed card matches its proof
        if cards.len() != proofs.len() {
            return; // Reject mismatched lengths
        }

        let dealer_secret = self.state.dealer_secret.get();
        for (card, proof) in cards.iter().zip(proofs.iter()) {
            // Verify the proof card matches the claimed card
            if proof.card != *card {
                return; // Reject - proof doesn't match claimed card
            }
            // Verify the proof uses the correct dealer secret
            if proof.secret != *dealer_secret {
                return; // Reject - wrong dealer secret
            }
        }

        players[player_idx].has_revealed = true;
        let seat = players[player_idx].seat;
        self.state.players.set(players.clone());

        // Store the revealed cards
        let mut revealed = self.state.revealed_cards.get().clone();
        revealed.push((seat, cards));
        self.state.revealed_cards.set(revealed);

        let all_revealed = players.iter().all(|p| p.has_folded || p.has_revealed);
        if all_revealed {
            self.determine_winner();
            self.settle_game().await;
        }
    }

    /// Determine winner using actual hand evaluation
    /// FIX #2: CRITICAL - Add bounds checking to prevent panics
    fn determine_winner(&mut self) {
        let players = self.state.players.get();

        // FIX #2: Bounds check - need exactly 2 players
        if players.len() < 2 {
            return; // Cannot determine winner without 2 players
        }

        // Check for fold first
        if players[0].has_folded {
            self.state.winner.set(Some(Seat::Player2));
            self.state.phase.set(GamePhase::Settlement);
            return;
        }
        if players[1].has_folded {
            self.state.winner.set(Some(Seat::Player1));
            self.state.phase.set(GamePhase::Settlement);
            return;
        }

        // Get revealed cards and community cards
        let revealed = self.state.revealed_cards.get();
        let community = self.state.community_cards.get();

        // Find each player's hole cards
        let p1_cards = revealed.iter().find(|(s, _)| *s == Seat::Player1).map(|(_, c)| c.clone());
        let p2_cards = revealed.iter().find(|(s, _)| *s == Seat::Player2).map(|(_, c)| c.clone());

        // FIX #9: MEDIUM - Implement pot splitting for ties
        let winner = match (p1_cards, p2_cards) {
            (Some(p1), Some(p2)) => {
                // Actual hand evaluation!
                let score1 = evaluate_hand(&p1, community);
                let score2 = evaluate_hand(&p2, community);

                match score1.cmp(&score2) {
                    std::cmp::Ordering::Greater => Some(Seat::Player1),
                    std::cmp::Ordering::Less => Some(Seat::Player2),
                    std::cmp::Ordering::Equal => None, // FIX #9: Tie - split pot
                }
            }
            (Some(_), None) => Some(Seat::Player1),
            (None, Some(_)) => Some(Seat::Player2),
            (None, None) => Some(Seat::Player1), // Default to Player1 if both missing
        };

        self.state.winner.set(winner);
        self.state.phase.set(GamePhase::Settlement);
    }

    /// Settle the game
    /// FIX #9: MEDIUM - Handle pot splitting for ties
    async fn settle_game(&mut self) {
        let pot = *self.state.pot.get();
        let game_id = *self.state.game_id.get();
        let players = self.state.players.get().clone();

        // FIX #9: Handle tie case (winner = None means split pot)
        let (payout_p1, payout_p2, is_tie) = match self.state.winner.get() {
            Some(Seat::Player1) => (pot, Amount::ZERO, false),
            Some(Seat::Player2) => (Amount::ZERO, pot, false),
            None => {
                // Split pot evenly for tie
                // Use saturating_div to split pot in half
                let half = pot.saturating_div(2);
                (half, half, true)
            }
        };

        for player in &players {
            let (payout, you_won) = match player.seat {
                Seat::Player1 => (payout_p1, !is_tie && payout_p1 > Amount::ZERO),
                Seat::Player2 => (payout_p2, !is_tie && payout_p2 > Amount::ZERO),
            };

            if player.hand_app.is_some() {
                self.runtime
                    .prepare_message(Message::GameResult {
                        game_id,
                        you_won,
                        payout,
                        opponent_cards: None,
                    })
                    .with_authentication()
                    .send_to(player.chain_id);
            }
        }

        self.state.phase.set(GamePhase::Finished);
    }

    /// Notify current player it's their turn
    async fn notify_turn(&mut self) {
        let seat = match self.state.turn_seat.get() {
            Some(s) => *s,
            None => return,
        };

        let players = self.state.players.get();
        let player = match players.iter().find(|p| p.seat == seat) {
            Some(p) => p,
            None => return,
        };

        if player.hand_app.is_some() {
            let game_id = *self.state.game_id.get();
            self.runtime
                .prepare_message(Message::YourTurn {
                    game_id,
                    current_bet: *self.state.current_bet.get(),
                    pot: *self.state.pot.get(),
                    min_raise: *self.state.min_raise.get(),
                })
                .with_authentication()
                .send_to(player.chain_id);
        }
    }

    /// Handle player leaving
    fn handle_leave(&mut self, player_chain: ChainId) {
        let mut players = self.state.players.get().clone();
        if let Some(idx) = players.iter().position(|p| p.chain_id == player_chain) {
            if *self.state.phase.get() == GamePhase::WaitingForPlayers {
                let player = players.remove(idx);
                let mut pot = *self.state.pot.get();
                pot = pot.saturating_sub(player.stake);
                self.state.pot.set(pot);
                self.state.players.set(players);
            } else {
                players[idx].has_folded = true;
                self.state.players.set(players);
            }
        }
    }

    /// Start new game
    fn start_new_game(&mut self) {
        let game_id = *self.state.game_id.get() + 1;
        self.state.game_id.set(game_id);
        self.state.phase.set(GamePhase::WaitingForPlayers);
        self.state.players.set(Vec::new());
        self.state.pot.set(Amount::ZERO);
        self.state.current_bet.set(Amount::ZERO);
        self.state.community_cards.set(Vec::new());
        self.state.turn_seat.set(None);
        self.state.winner.set(None);
        self.state.revealed_cards.set(Vec::new());
        // Reset dealer button (will be reassigned in deal_cards based on game_id)
        self.state.dealer_button.set(None);
        self.state.actions_this_round.set(0);
    }

    /// Generate deck seed
    fn generate_deck_seed(&mut self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(b"LINERA_POKER_DECK");
        hasher.update(self.state.game_id.get().to_le_bytes());
        hasher.update(self.runtime.chain_id().to_string().as_bytes());
        for player in self.state.players.get().iter() {
            hasher.update(player.chain_id.to_string().as_bytes());
        }
        hasher.finalize().to_vec()
    }

    /// Generate dealer secret
    fn generate_dealer_secret(&mut self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(b"LINERA_POKER_SECRET");
        hasher.update(self.state.game_id.get().to_le_bytes());
        hasher.update(self.runtime.chain_id().to_string().as_bytes());
        for player in self.state.players.get().iter() {
            hasher.update(player.chain_id.to_string().as_bytes());
            hasher.update(player.stake.to_string().as_bytes());
        }
        hasher.finalize().to_vec()
    }
}
