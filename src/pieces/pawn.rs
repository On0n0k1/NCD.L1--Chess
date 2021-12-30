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
pub struct Pawn{
    color: Color,
    current_column: u8,
    current_row: u8,
    has_moved: bool,
}

// impl Pawn{
//     fn include_if_has_rival(&self, target_column: i8, target_row: i8, board: &Board, report: &mut Report){
//         // let self_color: &Color = &self.color;
//         if(target_column < 0) || (target_row < 0) || (target_column > 7) || (target_column > 7){
//             // target_column or target_row is outside board's boundaries.
//             return;
//         }

//         let target_piece: Piece = board.get_piece(
//             target_column as u8,    // col: u8,
//             target_row as u8,       // row: u8,
//         );

//         if self.are_rivals(
//             &target_piece,          // target: &Piece,
//         ){
//             if target_piece.is_king(){
//                 report.set_check();
//             }

//             report.include_step(
//                 target_piece,       // target: Piece,
//             );
//         };
//     }

//     fn include_if_has_empty(&self, target_column: i8, target_row: i8, board: &Board, report: &mut Report){
//         // let self_color: &Color = &self.color;
//         if(target_column < 0) || (target_row < 0) || (target_column > 7) || (target_column > 7){
//             // target_column or target_row is outside board's boundaries.
//             return;
//         }

//         let target_piece: Piece = board.get_piece(
//             target_column as u8,                // col: u8,
//             target_row as u8,                   // row: u8,
//         );

//         if target_piece.is_color(&Color::EMPTY){

//             report.include_step(
//                 target_piece,                   // target: Piece,
//             );
//         }
//     }
    
// }


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
            has_moved: false,
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
        
        log(&format!("Getting Pawn Report"));
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

        // let front_step = col as i8 + step;
        let front_step = row as i8 + step;

        // Check if the position in front of the pawn is empty.
        // self.include_if_has_empty(
        //     front_step,                             // target_column: i8,
        //     row as i8,                              // target_row: i8,
        //     board,                                  // board: &Board,
        //     &mut report,                            // report: &mut Report,
        // );
        report.include_step(
            board,                                  // board: &Board, 
            col as i8,                              // target_column: i8, 
            front_step,                             // target_row: i8, 
            false,                                  // no_empty: bool, 
            true,                                   // no_rival: bool,
        );
        
        if !(self.has_moved) {
            // If the pawn never moved, it can take an extra step forward.
            // self.include_if_has_empty(
            //     front_step + step,                      // target_column: i8,
            //     row as i8,                              // target_row: i8,
            //     board,                                  // board: &Board,
            //     &mut report,                            // report: &mut Report,
            // );
            report.include_step(
                board,                              // board: &Board, 
                col as i8,                          // target_column: i8, 
                front_step + step,                  // target_row: i8, 
                false,                              // no_empty: bool, 
                true,                               // no_rival: bool,
            );
        }
        
        // Pawn can occupy the position in it's diagonals if a rival is occupying it.
        // self.include_if_has_rival(
        //     front_step,                             // target_column: i8,
        //     row as i8 + 1,                          // target_row: i8,
        //     board,                                  // board: &Board,
        //     &mut report,                            // report: &mut Report,
        // );
        report.include_step(
            board,                                  // board: &Board, 
            col as i8 +1,                           // target_column: i8, 
            front_step,                             // target_row: i8, 
            true,                                   // no_empty: bool, 
            false,                                  // no_rival: bool,
        );

        // self.include_if_has_rival(
        //     front_step,                             // target_column: i8,
        //     row as i8 - 1,                          // target_row: i8,
        //     board,                                  // board: &Board,
        //     &mut report,                            // report: &mut Report,
        // );
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
