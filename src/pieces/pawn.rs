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
pub struct Pawn{
    color: Color,
    current_column: u8,
    current_row: u8,
}

impl Pawn{
    fn is_first_move(&self) -> bool {

        return match self.color{
            Color::WHITE => {
                self.current_row == 6
            },
            Color::BLACK => {
                self.current_row == 1
            },
            _ => {
                panic!("Error in Pawn.is_first_move. Own color is Empty.")
            }
        }
    }
}


impl PieceExt for Pawn{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        let color: Color;
        if piece_number == 1 {
            color = Color::WHITE;
        } else if piece_number == 7 {
            color = Color::BLACK;
        } else {
            panic!("Error in Pawn Constructor. Attempted to create piece with invalid number({}).", piece_number);
        }
        Pawn{
            color,
            current_column: column,
            current_row: row,
        }
    }

    fn get_piece_number(&self) -> u8{
        let name: &str = "pawn";

        match self.color{
            Color::WHITE => 1,
            Color::BLACK => 7,
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
        
        // log(&format!("Getting Pawn Report"));
        let mut report: Report = Report::new(
            Piece::PAWN(self.clone()),          // piece: Piece,
            search_checkmate,                   // search_checkmate: bool,
        );

        let color: &Color = &self.color;
        
        let col = self.get_column();
        let row = self.get_row();
        
        // Black on top
        // White bottom
        let step: i8 = match color{
            Color::EMPTY => { panic!("Invalid color for pawn in get_valid_positions. Value is EMPTY."); },
            Color::BLACK => { 1 },
            Color::WHITE => { -1 },
        };

        let front_step = row as i8 + step;

        report.include_step(
            board,                                  // board: &Board, 
            col as i8,                              // target_column: i8, 
            front_step,                             // target_row: i8, 
            false,                                  // no_empty: bool, 
            true,                                   // no_rival: bool,
        );
        
        if self.is_first_move() {
            // If the pawn never moved, it can take an extra step forward.
            report.include_step(
                board,                              // board: &Board, 
                col as i8,                          // target_column: i8, 
                front_step + step,                  // target_row: i8, 
                false,                              // no_empty: bool, 
                true,                               // no_rival: bool,
            );
        }
        
        report.include_step(
            board,                                  // board: &Board, 
            col as i8 +1,                           // target_column: i8, 
            front_step,                             // target_row: i8, 
            true,                                   // no_empty: bool, 
            false,                                  // no_rival: bool,
        );

        report.include_step(
            board,                                  // board: &Board, 
            col as i8 - 1,                          // target_column: i8, 
            front_step,                             // target_row: i8, 
            true,                                   // no_empty: bool, 
            false,                                  // no_rival: bool,
        );

        report
    }
}
