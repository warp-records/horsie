

enum Color { Black, White }

enum PieceType {
   King(Color),
   Queen(Color),
   Rook(Color),
   Bishop(Color),
   Horses(Color),
   Pawn(Color, bool)
}

pub struct Game {
    black: ColorSet,
    white: ColorSet,
    turn: Color,
    win: Option<Color>,
}

/// struct representing board state of a given color
struct ColorSet {
    king: u64,
    queens: u64,
    rooks: u64,
    bishops: u64,
    horses: u64,
    pawns: u64,
    // bitboard representing any pawn which previously moved two squares
    en_passe: u64,
    moved_rooks: u64,
    king_moved: bool,
}
