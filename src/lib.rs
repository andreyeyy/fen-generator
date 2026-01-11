use rand::Rng;

const N_SQUARES: usize = 64;

#[derive(Debug)]
enum Color {
    White,
    Black,
}

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

struct Piece {
    piece_type: PieceType,
    color: Color,
}

struct Board {
    squares: [Option<Piece>; N_SQUARES],
    turn: Color,
}



#[inline]
/// Takes a file and a rank (0..8) and returns a square number (0..64)
fn board_index(file: usize, rank: usize) -> usize {
    debug_assert!(
        file < 8 && rank < 8,
        "square indices out of bounds: file={}, rank={}", file, rank);

    (8 * rank + file)
}

#[inline]
/// Takes an index (0..64) and returns (file, rank)
fn board_index_reverse(index: usize) -> (usize, usize)  {
    debug_assert!(
        index < 64,
        "index out of bounds: index={}", index);

    (index % 8, index / 8)
}


impl Board {
    /// Creates new empty board
    fn new() -> Self {
        Board {
            squares: [const { None }; N_SQUARES],
            turn: Color::White,
        }
    }

    /// Generates and returns a random board
    fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        const KING_MOVES: [(i8, i8); 8] = [
            (1, 1),
            (1, -1),
            (1, 0),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, -1),
            (-1, 0),
        ];
        let mut board = Board::new();
        board.turn = match rng.random_bool(0.5){
            true => Color::White,
            false => Color::Black,
        };

        let mut white_king_pos: usize = 0;
        let mut black_king_pos: usize = 0;
        { // Here we start placing the kings
            let mut occupied_squares = [false; N_SQUARES];

            //First randomly place white king
            white_king_pos = rng.random_range(0..N_SQUARES) as usize;
            

            board.squares[white_king_pos] = Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            });

            let (white_king_file, white_king_rank) = board_index_reverse(white_king_pos);

            occupied_squares[white_king_pos] = true;
            let mut free_squares_n: i8 = N_SQUARES as i8 - 1; // one square is already occupied by white king

            for &delta in &KING_MOVES { // fill in the occupied_squares array
                
                let curr_file = white_king_file as i8 + delta.0;
                let curr_rank = white_king_rank as i8 + delta.1;

                if curr_file < 0 || curr_rank < 0 {continue;}
                if curr_file >= 8 || curr_rank >= 8 {continue;}

                occupied_squares[board_index(curr_file as usize, curr_rank as usize)] = true;
                free_squares_n -= 1;
            }

            // -4 because the white king already occupies at least 4 squares (when in corner)
            let mut free_square_indexes: [usize; N_SQUARES - 4] = [0; N_SQUARES - 4]; 

            // j counts the current index of free_square_indexes 
            let mut j: usize = 0;

            for i in 0..N_SQUARES {
                if occupied_squares[i] {continue;} 

                free_square_indexes[j] = i;
                j += 1;
            }

            black_king_pos = free_square_indexes[rng.random_range(0..free_squares_n) as usize];

            board.squares[black_king_pos] = Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            });

        } // Here we end placing the kings
        
        // TODO: place the rest of the pieces

        return board;
    }
    
    /// Returns the fen representation of the board
    fn to_str_fen(&self) -> String{
        let mut fen = String::new();
        for rank in (0..8).rev() {

            let mut empty_squares_count = 0;

            for file in 0..8 {
                let square = &self.squares[board_index(file, rank)];

                match square {
                    Some(piece_type) => {
                        if empty_squares_count != 0 {
                            fen.push_str(&empty_squares_count.to_string());
                            empty_squares_count = 0;
                        }
                        fen.push(piece_type.to_char() as char);
                    },
                    None => {
                        empty_squares_count += 1;
                    },
                }

            }
            if empty_squares_count != 0 {
                fen.push_str(&empty_squares_count.to_string());

            }
            if rank == 0 {break}
            fen.push('/');
        }

        fen.push(' ');
        fen.push(match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        });

        fen.push_str(" - - 0 1");

        return fen;
    }

    /// Displays the board as a 2d image
    fn to_str(&self) -> String{
        let mut s = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = &self.squares[board_index(file, rank)];

                match square {
                    Some(piece_type) => s.push(piece_type.to_char() as char),
                    None => s.push('.'),
                }
            }
            s.push('\n');
        }
        s.push_str(&format!("Turn: {:?}", self.turn));
        return s;
    }
}

impl Piece {
    /// Returns the corresponding ASCII character of the Piece.
    /// For example Pawn would return b'p'
    fn to_char(&self) -> u8 {
        let letter = match self.piece_type {
            PieceType::Pawn => b'p',
            PieceType::Rook => b'r',
            PieceType::Knight => b'n',
            PieceType::Bishop => b'b',
            PieceType::Queen => b'q',
            PieceType::King => b'k',
        };
        match self.color {
            Color::White => letter.to_ascii_uppercase(),
            Color::Black => letter,
        }
    }
}

pub fn random_fen() -> String {
    let mut rng = rand::rng();
    let board = Board::random(&mut rng);
    board.to_str_fen()
}


