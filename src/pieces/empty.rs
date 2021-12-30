use std::clone::Clone;

use crate::{
    // log,
    board::Board,
    pieces::{
        piece::{
            Color,
            // Piece,
        },
        piece_ext::PieceExt,
    },
    movement::report::Report,
};


#[derive(Clone, Copy, PartialEq)]
pub struct Empty{
    current_column: u8,
    current_row: u8,
}


impl PieceExt for Empty{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        if piece_number != 0 {
            panic!("Tried to create Empty Piece with number {} ", piece_number);
        }

        Empty{
            current_column: column,
            current_row: row,
        }
    }

    fn get_piece_number(&self) -> u8{
        0
    }

    fn get_column(&self) -> u8 {
        return self.current_column;
    }

    fn get_row(&self) -> u8 {
        return self.current_row;
    }

    fn get_color(&self) -> Color{
        Color::EMPTY
    }

    fn is_king(&self) -> bool {
        false
    }

    fn get_movement_report(
        &self,
        _board: &Board, 
        _search_checkmate: bool,
    ) -> Report {           

       panic!("Tried to get Movement Report from an empty Position");
    }

}
