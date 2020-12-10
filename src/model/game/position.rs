use crate::model::game::square;
use crate::model::game::token;

#[derive(Copy, Clone)]
pub struct Position {
    board: [token::Token; 9],
}

impl Position {
    fn empty() -> Position {
        Position {
            board: [token::Token::Empty; 9],
        }
    }
}

#[derive(Debug)]
pub struct PositionError {
    details: String,
}

impl PositionError {
    fn new(msg: &str) -> PositionError {
        PositionError{details: msg.to_string()}
    }
}

fn square_to_index(square: square::Square) -> usize {
    return match square {
        square::Square::A1 => 0,
        square::Square::A2 => 1,
        square::Square::A3 => 2,
        square::Square::B1 => 3,
        square::Square::B2 => 4,
        square::Square::B3 => 5,
        square::Square::C1 => 6,
        square::Square::C2 => 7,
        square::Square::C3 => 8,
    };
}

pub fn winner(position: Position) -> token::Token {
    // From A1
    if position.board[0] != token::Token::Empty {
        if position.board[0] == position.board[1] && position.board[0] == position.board[2] {
            return position.board[0];
        }

        if position.board[0] == position.board[3] && position.board[0] == position.board[6] {
            return position.board[0];
        }

        if position.board[0] == position.board[4] && position.board[0] == position.board[8] {
            return position.board[0];
        }
    }

    // From A2
    if position.board[1] != token::Token::Empty {
        if position.board[1] == position.board[4] && position.board[1] == position.board[7] {
            return position.board[1];
        }
    }

    // From A3
    if position.board[2] != token::Token::Empty {
        if position.board[2] == position.board[5] && position.board[2] == position.board[7] {
            return position.board[2];
        }

        if position.board[2] == position.board[4] && position.board[2] == position.board[6] {
            return position.board[2];
        }
    }

    // From B1 
    if position.board[3] != token::Token::Empty {
        if position.board[3] == position.board[4] && position.board[3] == position.board[5] {
            return position.board[3];
        }
    }

    // From C1
    if position.board[6] != token::Token::Empty {
        if position.board[6] == position.board[7] && position.board[6] == position.board[8] {
            return position.board[6];
        }
    }

    return token::Token::Empty;
}

pub fn apply_move(position: Position, square: square::Square, token: token::Token) -> Result<Position, PositionError> {
    let i = square_to_index(square);

    return match position.board[i] {
        token::Token::Empty =>{

            let mut new_board = position.board.clone();
            new_board[i] = token;

            return Ok(
                Position {
                    board: new_board,
                }
            );
        }
        _ => Err(PositionError::new("Cannot apply move to non-empty square")),
    }
}

#[cfg(test)]
mod tests {
    use crate::model::game::position;
    use crate::model::game::square;
    use crate::model::game::token;

    #[test]
    fn empty_position_has_no_winner() {
        let pos = position::Position::empty();

        assert_eq!(position::winner(pos), token::Token::Empty);
    }

    #[test]
    fn x_wins_first_rank() -> Result<(), position::PositionError> {
        let empty = position::Position::empty();
        let x = position::apply_move(empty, square::Square::A1, token::Token::X)?;
        let xx = position::apply_move(x, square::Square::B1, token::Token::X)?;
        let xxx = position::apply_move(xx, square::Square::C1, token::Token::X)?;

        assert_eq!(position::winner(xxx), token::Token::X);

        return Ok(());
    }

    #[test]
    fn o_wins_a_file() -> Result<(), position::PositionError> {
        let empty = position::Position::empty();
        let o = position::apply_move(empty, square::Square::A1, token::Token::O)?;
        let oo = position::apply_move(o, square::Square::A2, token::Token::O)?;
        let ooo = position::apply_move(oo, square::Square::A3, token::Token::O)?;

        assert_eq!(position::winner(ooo), token::Token::O);

        return Ok(());
    }

    #[test]
    fn x_wins_a3_daigonal() -> Result<(), position::PositionError> {
        let empty = position::Position::empty();
        let x = position::apply_move(empty, square::Square::A3, token::Token::X)?;
        let xx = position::apply_move(x, square::Square::B2, token::Token::X)?;
        let xxx = position::apply_move(xx, square::Square::C1, token::Token::X)?;

        assert_eq!(position::winner(xxx), token::Token::X);

        return Ok(());
    }

    #[test]
    fn x_wins_c3_diagonal() -> Result<(), position::PositionError> {
        let empty = position::Position::empty();
        let x = position::apply_move(empty, square::Square::C3, token::Token::X)?;
        let xx = position::apply_move(x, square::Square::B2, token::Token::X)?;
        let xxx = position::apply_move(xx, square::Square::A1, token::Token::X)?;

        assert_eq!(position::winner(xxx), token::Token::X);

        return Ok(());
    }
}
