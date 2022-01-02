use std::clone::Clone;

use crate::{
    log,
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
pub struct Bishop{
    color: Color,
    current_column: u8,
    current_row: u8,
}


impl PieceExt for Bishop{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        let color: Color;
        if piece_number == 4 {
            color = Color::WHITE;
        } else if piece_number == 10 {
            color = Color::BLACK;
        } else {
            panic!("Error in Bishop Constructor. Attempted to create piece with invalid number({}).", piece_number);
        }
        // assert!((piece_number==4) || (piece_number==10), "Error in Bishop Constructor. Attempted to create piece with invalid number({}).", piece_number);
        Bishop{
            color,
            current_column: column,
            current_row: row,
        }
    }

    fn get_piece_number(&self) -> u8{
        let name: &str = "bishop";
        match self.color{
            Color::WHITE => 4,
            Color::BLACK => 10,
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
        log(&format!("Getting Bishop Report"));
        let mut report: Report = Report::new(
            Piece::BISHOP(self.clone()),// piece: Piece, 
            search_checkmate,           // search_checkmate: bool,
        );

        report.apply_valid_diagonal_positions(
            7,                 // max_steps: i8, 
            &board,                     // board: &Board,
        );

        report
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn
// }
