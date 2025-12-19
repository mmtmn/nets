use super::board::{Board, Piece};
use super::r#move::ChessMove;

pub fn is_legal(board: &Board, mv: ChessMove) -> bool {
    let from = mv.from as usize;
    let to = mv.to as usize;

    if from >= 64 || to >= 64 { return false; }

    let p = board.squares[from];
    if p == Piece::Empty { return false; }

    // side to move
    let is_white = matches!(p, Piece::WP|Piece::WN|Piece::WB|Piece::WR|Piece::WQ|Piece::WK);
    if is_white != board.white_to_move { return false; }

    // cannot capture own piece
    let target = board.squares[to];
    if target != Piece::Empty {
        let target_white = matches!(target, Piece::WP|Piece::WN|Piece::WB|Piece::WR|Piece::WQ|Piece::WK);
        if target_white == is_white { return false; }
    }

    // v0 legality: allow any move of own piece
    // (this is intentional minimalism; illegal moves are still detectable)
    true
}

pub fn apply(board: &mut Board, mv: ChessMove) {
    let p = board.squares[mv.from as usize];
    board.squares[mv.from as usize] = Piece::Empty;
    board.squares[mv.to as usize] = p;
    board.white_to_move = !board.white_to_move;
}

pub fn has_king(board: &Board, white: bool) -> bool {
    board.squares.iter().any(|p| {
        if white { *p == Piece::WK } else { *p == Piece::BK }
    })
}
