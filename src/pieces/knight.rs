use crate::{
    board::Board,
    log,
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
pub struct Knight{
    color: Color,
    current_column: u8,
    current_row: u8,
}


impl PieceExt for Knight{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        let color: Color;
        if piece_number == 3 {
            color = Color::WHITE;
        } else if piece_number == 9 {
            color = Color::BLACK;
        } else {
            panic!("Error in Knight Constructor. Attempted to create piece with invalid number({}).", piece_number);
        }

        Knight{
            color,
            current_column: column,
            current_row: row,
        }
    }

    fn get_piece_number(&self) -> u8{
        let name: &str = "knight";
        match self.color{
            Color::WHITE => 3,
            Color::BLACK => 9,
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
        // let mut valid_positions: Vec<u8> = Vec::with_capacity(8);
            
        log(&format!("Getting Knight report"));
        let mut report: Report= Report::new(
            Piece::KNIGHT(self.clone()),    // piece: Piece,
            search_checkmate,               // search_checkmate: bool,
        );

        let row: i8 = self.current_row as i8;
        let col: i8 = self.current_column as i8;
        
        // X: Position in the board.
        // K: Where knight is located.
        // 1 to 8: All positions where knight can move.

        // - - - - - - - -
        // - - - - - - - -
        // - - - 8 - 7 - -
        // - - 4 - - - 2 -
        // - - - - K - - -
        // - - 3 - - - 1 -
        // - - - 6 - 5 - -
        // - - - - - - - -


        // This are all the possible positions that the knight can try to occupy.
        let position_candidates: [(i8, i8); 8] = [
            (col + 2, row + 1),             // 1
            (col + 2, row - 1),             // 2
            (col - 2, row + 1),             // 3
            (col - 2, row - 1),             // 4
            (col + 1, row + 2),             // 5
            (col - 1, row + 2),             // 6
            (col + 1, row - 2),             // 7
            (col - 1, row - 2),             // 8
        ];
        position_candidates.map(|(col, row)| {

            report.include_step(
                board,                      // board: &Board, 
                col,                        // target_column: i8, 
                row,                        // target_row: i8, 
                false,                      // no_empty: bool, 
                false,                      // no_rival: bool,
            );
        });

        report
    }
}