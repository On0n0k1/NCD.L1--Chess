use std::clone::Clone;

use crate::{
    board::Board,
    log,
    movement::report::Report,
    pieces::{
        piece::{
            Color,
            Piece,
        },
        piece_ext::PieceExt,
    },
};


#[derive(Clone, Copy, PartialEq)]
pub struct Queen{
    color: Color,
    current_column: u8,
    current_row: u8,
}


impl PieceExt for Queen{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        let color: Color;
        if piece_number == 5 {
            color = Color::WHITE;
        } else if piece_number == 11 {
            color = Color::BLACK;
        } else {
            panic!("Error in Queen Constructor. Attempted to create piece with invalid number({}).", piece_number);
        }
        Queen{
            color,
            current_column: column,
            current_row: row,
        }
    }

    fn get_piece_number(&self) -> u8{
        let name: &str = "queen";
        
        match self.color{
            Color::WHITE => 5,
            Color::BLACK => 11,
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
        false
    }

    fn get_movement_report(
        &self,
        board: &Board, 
        search_checkmate: bool,
    ) -> Report {            
        log(&format!("Getting Queen Report"));
        let mut report: Report = Report::new(
            Piece::QUEEN(self.clone()),     // piece: Piece,
            search_checkmate,               // search_checkmate: bool,
        );
        
        // Queen moves like the rook...
        report.apply_valid_ortogonal_positions(
            7,                              // max_steps: i8, 
            board,                          // board: &Board,
        );

        // and also like the bishop.
        report.apply_valid_diagonal_positions(
            7,                              // max_steps: i8, 
            board,                          // board: &Board,
        );

        report
    }
}

