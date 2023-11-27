use std::{ io::{ self, Write }, str::FromStr, cmp::min, cmp::max, collections::HashMap };
use shakmaty::{ Chess, Move, Color, Role, Square, File, Rank, Position };
use shakmaty::uci::Uci;

fn evaluate_pos(pos: &Chess) -> i32 {
    let mut score = 0;

    // Piece-square table
    let piece_square_table: HashMap<(Role, Square), i32> = [
        ((Role::Pawn, Square::A4), 1),
        ((Role::Pawn, Square::D4), 1),
        ((Role::Pawn, Square::E4), 1),
        ((Role::Pawn, Square::H4), 1),
        // Add more entries for other pieces and squares
    ]
        .iter()
        .cloned()
        .collect();

    for sq in Square::ALL {
        if let Some(piece) = pos.board().piece_at(sq) {
            let mut value = match piece.role {
                Role::Pawn => 1,
                Role::Knight => 3,
                Role::Bishop => 3,
                Role::Rook => 5,
                Role::Queen => 9,
                Role::King => 0,
            };

            // Add piece-square table value
            if let Some(piece_square_value) = piece_square_table.get(&(piece.role, sq)) {
                value += *piece_square_value;
            }

            if piece.color == Color::White {
                score += value;
            } else {
                score -= value;
            }
        }
    }

    // Mobility
    let mobility = pos.legal_moves().len() as i32;
    score += mobility;

    score
}

fn alpha_beta_search(
    pos: &Chess,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool
) -> (Option<Move>, i32) {
    if depth == 0 || pos.is_checkmate() || pos.is_stalemate() {
        return (None, evaluate_pos(pos));
    }

    let mut best_move = None;
    let mut value;
    let moves = pos.legal_moves();

    if maximizing_player {
        value = i32::MIN;
        for m in &moves {
            let mut new_pos = pos.clone();
            new_pos.play_unchecked(&m);
            let (_, move_value) = alpha_beta_search(&new_pos, depth - 1, alpha, beta, false);
            if move_value > value {
                value = move_value;
                best_move = Some(m);
            }
            alpha = max(alpha, value);
            if alpha >= beta {
                break; // Beta cut-off
            }
        }
    } else {
        value = i32::MAX;
        for m in &moves {
            let mut new_pos = pos.clone();
            new_pos.play_unchecked(&m);
            let (_, move_value) = alpha_beta_search(&new_pos, depth - 1, alpha, beta, true);
            if move_value < value {
                value = move_value;
                best_move = Some(m);
            }
            beta = min(beta, value);
            if beta <= alpha {
                break; // Alpha cut-off
            }
        }
    }

    (best_move.cloned(), value)
}

fn print_board(pos: &Chess) {
    println!("  a b c d e f g h");

    for rank in (0..8).rev() {
        print!("{} ", rank + 1);

        for file in 0..8 {
            let square = Square::from_coords(
                File::try_from(file).unwrap(),
                Rank::try_from(rank).unwrap()
            );
            match pos.board().piece_at(square) {
                Some(piece) => {
                    let ch = match piece.role {
                        Role::Pawn => if piece.color == Color::Black { '♙' } else { '♟' }
                        Role::Knight => if piece.color == Color::Black { '♘' } else { '♞' }
                        Role::Bishop => if piece.color == Color::Black { '♗' } else { '♝' }
                        Role::Rook => if piece.color == Color::Black { '♖' } else { '♜' }
                        Role::Queen => if piece.color == Color::Black { '♕' } else { '♛' }
                        Role::King => if piece.color == Color::Black { '♔' } else { '♚' }
                    };
                    print!("{}", ch);
                }
                None => print!("."),
            }
        }
        println!(" {}", rank + 1);
    }

    println!("  a b c d e f g h");
}

fn main() {
    let mut pos = Chess::default();

    while !pos.is_checkmate() && !pos.is_stalemate() {
        print_board(&pos);

        // Player's move
        print!("Enter your move: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let user_move = Uci::from_str(input);
        match user_move {
            Ok(uci) => {
                let move_ = uci.to_move(&pos);
                match move_ {
                    Ok(move_) => {
                        if pos.is_legal(&move_) {
                            pos = pos.play(&move_).expect("move should be legal");
                        } else {
                            println!("Illegal move");
                            continue;
                        }
                    }
                    Err(_) => {
                        println!("Invalid move");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Invalid move");
                continue;
            }
        }

        // Bot's move
        let (best_move, _) = alpha_beta_search(&pos, 6, i32::MIN, i32::MAX, false);
        if let Some(best_move) = best_move {
            pos = pos.play(&best_move).expect("move should be legal");
        }
    }

    if pos.is_checkmate() {
        if pos.turn().is_white() {
            println!("Black wins by checkmate!");
        } else {
            println!("White wins by checkmate!");
        }
    }
}
