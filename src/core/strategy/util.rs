use crate::core::strategy::{Move, Position, Side::{self, *}, StrategyGrid};

/// Return the `Side` with the largest payoff from a `StrategyGrid`.
///
/// Tie-breaking follows the original code's lexical preference: AA, AB, BA, BB.
pub fn find_biggest_payoff(StrategyGrid { aa, ab, ba, bb, .. }: StrategyGrid) -> Side {
    [
        (AA, aa),
        (AB, ab),
        (BA, ba),
        (BB, bb),
    ]
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(side, _)| side)
        .unwrap_or(AA)

    // if aa>=ab && aa>=ba && aa>=bb {
    //     AA
    // } else if ab>=aa && ab>=ba && ab>=bb {
    //     AB
    // } else if ba>=aa && ba>=ab && ba>=bb {
    //     BA
    // } else {
    //     BB  
    // }
}

/// Choose the `Move` (A or B) that does best against the opponent's last
/// move according to the payoff entries in `StrategyGrid`.
///
/// This preserves the original behavior: when the two candidate payoffs are
/// equal the function returns `Move::B` (same as the original JS which used
/// `>` and chose 2 on ties).
pub fn what_beats_last(my_position: Position, board: &StrategyGrid) -> Move {
    // Determine the opponent's last move depending on their position.
    let opponent_prev = match my_position.opposite() {
        Position::H => board.previous_horizontal_move,
        Position::V => board.previous_vertical_move,
    };

    // If the opponent's previous move was A, compare `aa` vs `ba`.
    // Otherwise compare `ab` vs `bb`.
    let (lhs, rhs) = match opponent_prev {
        Move::A => (board.aa, board.ba),
        Move::B => (board.ab, board.bb),
    };

    if lhs > rhs { Move::A } else { Move::B }
}