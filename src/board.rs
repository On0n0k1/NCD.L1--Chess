use std::default::Default;

// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::{
    // This is just to call Report::col_row_to_index later.
    movement::report::Report,
    pieces::{
        piece::Color,
        piece::Piece,
        piece_ext::PieceExt,
    },
};

// use crate
// Need to implement a board struct for computation.

pub struct Board{
    squares: [u8; 64],
    built_squares: [Piece; 64],
    // turn: Player,
}

// empty:           0
// white pawn:      1
// white rook:      2
// white knight:    3
// white bishop:    4
// white queen:     5
// white king:      6
// black pawn:      7
// black rook:      8
// black knight:    9
// black bishop:    10
// black queen:     11
// black king:      12

// 8 9 A B C A 9 8 
// 7 7 7 7 7 7 7 7
// 0 0 0 0 0 0 0 0 
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 1 1 1 1 1 1 1 1 
// 2 3 4 5 6 4 3 2

// Board checks if king is in check
// If not, check if requested move is valid
// If move is valid, do the move, then check again if king is in check.
// If king is in check after the move, return false, 
// else commit the board state and return true.

impl Default for Board{
    fn default() -> Self{
        let squares: [u8; 64] = Board::get_default_board();

        let built_squares: [Piece; 64] = Board::new_built_squares(
            &squares,                           // squares: &[u8; 64]
        );
        
        Board{
            squares,
            built_squares,
        }
    }
}


impl Board{

    pub fn get_default_board() -> [u8; 64] {
        [
            8, 9, 10, 11, 12, 10, 9, 8,
            7, 7,  7,  7,  7,  7, 7, 7,
            0, 0,  0,  0,  0,  0, 0, 0,
            0, 0,  0,  0,  0,  0, 0, 0,
            0, 0,  0,  0,  0,  0, 0, 0,
            0, 0,  0,  0,  0,  0, 0, 0,
            1, 1,  1,  1,  1,  1, 1, 1,
            2, 3,  4,  5,  6,  4, 3, 2,
        ]
    }

    pub fn new_built_squares(squares: &[u8; 64]) -> [Piece; 64] {
        let mut built_squares:[Piece; 64] = [Piece::default(); 64];
        // An array created like this has 64 copies of the same value.
        // We need each of the EMPTY pieces to hold the correct column/row values.
        let (mut col, mut row): (u8, u8) = (0, 0);

        for index in 0..squares.len(){

            // built_squares[index].set_row(row);
            // built_squares[index].set_column(col);

            let empty_replacement: Piece = Piece::new(
                0,                      // piece_number: u8, 
                col,                    // column: u8, 
                row,                    // row: u8,
            );
            built_squares[index] = empty_replacement;

            col += 1;
            if col >= 8 {
                row += 1;
                col = 0;
            }
        }

        // Put the pieces in the board.
        Self::build(
            &squares,                   // squares: &[u8;64], 
            &mut built_squares,         // built_squares: &mut [Piece; 64],
        );

        built_squares
    }

    pub fn new(squares: [u8; 64]) -> Self {
        let built_squares: [Piece; 64] = Self::new_built_squares(
            &squares,                   // squares: &[u8; 64]
        );

        Board{
            squares,
            built_squares,
        }
    }

    pub fn get_board_array(&self) -> [u8; 64] {
        return self.squares.clone();
    }

    // Turn the board into a square of numbers
    // Instead of an array, we're using a byte slice
    // We're not checking for length because 
    // we want it to panic if there's less than 64 elements.
    // It should never ever have more or less than 64 values.
    pub fn get_board_string(squares: &[u8]) -> String {
        fn hex(number: u8) -> String {
            match number{
                10 => String::from("A"),
                11 => String::from("B"),
                12 => String::from("C"),
                13 => String::from("D"),
                14 => String::from("E"),
                15 => String::from("F"),
                _ => format!("{}", number),
            }
        }

        let mut response: String = String::from("\n");

        for row in 0..8{
            let index: usize = row * 8;
            response = format!("{}{} {} {} {} {} {} {} {}\n",
                response,
                hex(squares[index]),
                hex(squares[index + 1]),
                hex(squares[index + 2]),
                hex(squares[index + 3]),
                hex(squares[index + 4]),
                hex(squares[index + 5]),
                hex(squares[index + 6]),
                hex(squares[index + 7]),
            );
        }

        response = format!("{}\n", response);

        response
    }

    /// Checks the board array (squares) and update built_squares with any change that exists between the two.
    /// This is used in the first step of each move request.
    fn build(
        squares: &[u8;64], 
        built_squares: &mut [Piece; 64],
    ){
        for index in 0..squares.len(){
            let piece_number: u8 = squares[index];
            if piece_number > 12 {
                panic!("Invalid piece number on the board ({}).", piece_number);
            }

            // If piece_number is already the expected Piece type, ignore it.
            match (piece_number, &built_squares[index]){
                (0, Piece::EMPTY(_))  
                | (1, Piece::PAWN(_)  ) | (7,  Piece::PAWN(_)  )
                | (2, Piece::ROOK(_)  ) | (8,  Piece::ROOK(_)  )
                | (3, Piece::KNIGHT(_)) | (9,  Piece::KNIGHT(_))
                | (4, Piece::BISHOP(_)) | (10, Piece::BISHOP(_))
                | (5, Piece::QUEEN(_) ) | (11, Piece::QUEEN(_) )
                | (6, Piece::KING(_)  ) | (12, Piece::KING(_)  )   => { continue; },
                (_, _) => {
                    let column: u8 = (index % 8) as u8; 
                    let row: u8 = (index as u8 - column)/8;

                    // The number doesn't refer to the same type of piece.
                    // Therefore we replace it with the expected value.
                    built_squares[index] = Piece::new(
                        piece_number,   // piece_number: u8, 
                        column,         // column: u8, 
                        row,            // row: u8,
                    );
                }
            }
        }
    }

    pub fn get_piece(&self, col: u8, row: u8) -> Piece{
        // let piece: Piece = self.built_squares[(row * 8 + col) as usize].clone();
        let piece: Piece = self.get_piece_index(row * 8 + col);
        piece
    }

    pub fn get_piece_index(&self, index: u8) -> Piece{
        let piece: Piece = self.built_squares[index as usize].clone();
        piece
    }

    fn push_pieces(&self, pieces: &mut Vec<Piece>, color: &Color) {
        for index in 0..self.built_squares.len(){
            let piece: &Piece = &self.built_squares[index];

            if piece.is_color(
                color,                  // target_color: &Color,
            ){
                pieces.push(piece.clone());
            }
        }
    }

    pub fn push_black_pieces(&self, pieces: &mut Vec<Piece>) {
        pieces.clear();

        self.push_pieces(
            pieces,                     // pieces: &mut Vec<Piece>
            &Color::BLACK,              // color: &Color
        );
    }

    pub fn push_white_pieces(&self, pieces: &mut Vec<Piece>) {
        pieces.clear();

        self.push_pieces(
            pieces,                     // pieces: &mut Vec<Piece>
            &Color::WHITE,              // color: &Color
        )
    }

    pub fn replace_piece(
        &mut self,
        piece_number: u8,
        col: u8,
        row: u8,
    ) {
        let index: usize = Report::col_row_to_index(
            col,                        // col: u8, 
            row,                        // row: u8,
        ) as usize;

        self.built_squares[index] = Piece::new(
            piece_number,               // piece_number: u8, 
            col,                        // column: u8, 
            row,                        // row: u8,
        );
    }

    /// Does the same as replace_piece, but to the final state (squares) of the board.
    /// Used by step.
    pub fn finalize_movement(
        &mut self,
        piece_number: u8,
        col: u8,
        row: u8,
    ) {
        let index: usize = Report::col_row_to_index(
            col,                        // col: u8, 
            row,                        // row: u8,
        ) as usize;

        self.squares[index] = piece_number;


    }
}
