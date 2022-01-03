use std::clone::Clone;

use crate::{
    board::Board,
    pieces::{
        piece::{
            Color,
            Piece,
        },
        piece_ext::PieceExt,
    },
    movement::report::Report,
};


#[derive(Clone, Copy, PartialEq)]
pub struct King{
    color: Color,
    current_column: u8,
    current_row: u8,
}


impl PieceExt for King{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        let color: Color;
        if piece_number == 6 {
            color = Color::WHITE;
        } else if piece_number == 12 {
            color = Color::BLACK;
        } else {
            panic!("Error in King Constructor. Attempted to create piece with invalid number({}).", piece_number);
        }
        
        King{
            color,
            current_column: column,
            current_row: row,
        }
    }

    fn get_piece_number(&self) -> u8{
        let name: &str = "king";
        match self.color{
            Color::WHITE => 6,
            Color::BLACK => 12,
            Color::EMPTY => panic!("Invalid color for {}.get_piece_number. Own color is empty.", name),
        }
    }

    fn get_column(&self) -> u8 {
        return self.current_column;
    }

    fn get_row(&self) -> u8 {
        return self.current_row;
    }

    fn get_color(&self) -> Color{
        let color = self.color.clone();
        color
    }

    fn is_king(&self) -> bool {
        true
    }

    fn get_movement_report(
        &self,
        board: &Board, 
        search_checkmate: bool,
    ) -> Report {           

        // log(&format!("Getting King Report"));
        let mut report: Report = Report::new(
            Piece::KING(self.clone()),      // piece: Piece,
            search_checkmate                ,// search_checkmate: bool,
        );

        report.apply_valid_ortogonal_positions(
            1,                              // max_steps: i8,
            board,                          // board: &Board,
        );

        report.apply_valid_diagonal_positions(
            1,                              // max_steps: i8,
            board,                          // board: &Board,
        );

        if report.is_check() {
            panic!("Somehow the king managed to enable check. This shouldn't be possible.");
        }

        report
    }

}
