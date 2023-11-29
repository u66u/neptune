use shakmaty::{ Role, Color, Square, Chess, Position };

type Psqt = [i32; 64]; // Define the type of your piece-square table as a 2D array.

#[rustfmt::skip]
const KING_MG: Psqt = [
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,   -10,  -10,    0,    0,    0,
    0,    0,    20,   -10,  -10,    0,   20,    0,
];

#[rustfmt::skip]
const QUEEN_MG: Psqt = [
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -10,   -5,   10,   10,   10,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30 
];

#[rustfmt::skip]
const ROOK_MG: Psqt = [
    0,   0,   0,   0,   0,   0,   0,   0,
   15,  15,  15,  20,  20,  15,  15,  15,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,  10,  10,  10,   0,   0
];

#[rustfmt::skip]
const BISHOP_MG: Psqt = [
    -20,    0,    0,    0,    0,    0,    0,  -20,
    -15,    0,    0,    0,    0,    0,    0,  -15,
    -10,    0,    0,    5,    5,    0,    0,  -10,
    -10,   10,   10,   30,   30,   10,   10,  -10,
      5,    5,   10,   25,   25,   10,    5,    5,
      5,    5,    5,   10,   10,    5,    5,    5,
    -10,    5,    5,   10,   10,    5,    5,  -10,
    -20,  -10,  -10,  -10,  -10,  -10,  -10,  -20
];

#[rustfmt::skip]
const KNIGHT_MG: Psqt = [
    -20, -10,  -10,  -10,  -10,  -10,  -10,  -20,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   10,   15,   15,   15,   -5,  -10,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,   0,  -10,  -10,  -10,  -10,    0,  -20
];

#[rustfmt::skip]
const PAWN_MG: Psqt = [
     0,   0,   0,   0,   0,   0,   0,   0,
    60,  60,  60,  60,  70,  60,  60,  60,
    40,  40,  40,  50,  60,  40,  40,  40,
    20,  20,  20,  40,  50,  20,  20,  20,
     5,   5,  15,  30,  40,  10,   5,   5,
     5,   5,  10,  20,  30,   5,   5,   5,
     5,   5,   5, -30, -30,   5,   5,   5,
     0,   0,   0,   0,   0,   0,   0,   0
];

pub struct PieceSquareTable {
    pawn: Psqt,
    knight: Psqt,
    bishop: Psqt,
    rook: Psqt,
    queen: Psqt,
    king: Psqt,
}

impl PieceSquareTable {
    pub fn new() -> Self {
        Self {
            pawn: PAWN_MG,
            knight: KNIGHT_MG,
            bishop: BISHOP_MG,
            rook: ROOK_MG,
            queen: QUEEN_MG,
            king: KING_MG,
        }
    }
}

pub fn get_piece_value(pst: &PieceSquareTable, role: Role, color: Color, square: Square) -> i32 {
    let (file, rank) = (square.file() as usize, square.rank() as usize);
    let index = if color == Color::White { (7 - rank) * 8 + file } else { rank * 8 + file };

    let base_value = match role {
        Role::Pawn => 100,
        Role::Knight => 320,
        Role::Bishop => 330,
        Role::Rook => 500,
        Role::Queen => 900,
        Role::King => 20000,
        _ => 0,
    };

    let pst_value = match role {
        Role::Pawn => pst.pawn[index],
        Role::Knight => pst.knight[index],
        Role::Bishop => pst.bishop[index],
        Role::Rook => pst.rook[index],
        Role::Queen => pst.queen[index],
        Role::King => pst.king[index],
        _ => 0,
    };

    return base_value + pst_value;
}

pub fn evaluate_pos(pos: &Chess) -> i32 {
    let mut score = 0;
    let pst = PieceSquareTable::new();

    // Calculate the piece-square table score for all pieces
    for square in pos.board().occupied().into_iter() {
        let piece = pos.board().piece_at(square).unwrap();
        let value = get_piece_value(&pst, piece.role, piece.color, square);
        if piece.color == pos.turn() {
            score += value;
        } else {
            score -= value;
        }
    }

    score
}
