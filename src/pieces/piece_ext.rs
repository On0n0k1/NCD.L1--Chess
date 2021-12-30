use crate::{
    board::Board,
    pieces::{
        piece::{
            Color,
            Piece,
        }
    },
    movement::report::Report,
};


pub trait PieceExt{
    /// Create a new instance of this Piece. 
    fn new(
        piece_number: u8,
        target_column: u8,
        target_row: u8,
    ) -> Self;

    fn get_piece_number(&self) -> u8;

    fn get_column(&self) -> u8;

    fn get_row(&self) -> u8;

    fn get_color(&self) -> Color;

    fn is_king(&self) -> bool;

    /// Represents all the positions this piece may make.
    fn get_movement_report(&self, board: &Board, search_checkmate: bool) -> Report;

    fn are_rivals(&self, target: &Piece) -> bool {
        let target_color: Color = target.get_color();
        let self_color: Color = self.get_color();
        
        match(self_color, target_color) {
            (Color::BLACK, Color::WHITE) => return true,
            (Color::WHITE, Color::BLACK) => return true,
            (Color::EMPTY, _) => panic!("Error in function is_rival. self value is Empty. This shouldn't ever be checked."),
            (_, _) => return false,
        }
    }

    fn is_color(&self, target_color: &Color) -> bool {
        let self_color: Color = self.get_color();
        
        return self_color == *target_color
    }
}
