use crate::{
    board::Board,
    pieces::{
        piece::{
            Color,
            Piece,
        },
        piece_ext::PieceExt,
    },
    movement::step::Step,
};


/// Represents the movement report of a single piece.
#[derive(Clone)]
pub struct Report{
    piece: Piece,
    check: bool,
    steps: Vec<Step>,
    search_checkmate: bool,
}


impl Report{
    
    pub fn new(
        piece: Piece,
        search_checkmate: bool,
    ) -> Self {

        Report {
            piece: piece,
            check: false,
            steps: Vec::new(),
            search_checkmate,
        }
    }

    // pub fn new_search_checkmate(
    //     piece: Piece,
    // ) -> Self{

    //     Report {
    //         piece: piece,
    //         check: false,
    //         steps: Vec::new(),
    //         search_checkmate: true,
    //     }
    // }

    pub fn is_check(&self) -> bool{
        self.check.clone()
    }

    pub fn set_check(&mut self) {
        self.check = true;
    }

    // returns false if target is occupied by a rival or ally.
    pub fn include_step(
        &mut self, 
        board: &Board,
        target_column: i8,
        target_row: i8,
        // used by pawn. If true, it will not include empty positions.
        no_empty: bool,
        // used by pawn. If true, it will not include rival positions.
        no_rival: bool,
    ) -> bool {
        let current_color: Color = self.piece.get_color();

        if(target_column < 0) || (target_row < 0) || (target_column > 7) || (target_row > 7){
            // target_column or target_row is outside board's boundaries.
            return false;
        }

        let target_piece: Piece = board.get_piece(
            target_column as u8,                // col: u8,
            target_row as u8,                   // row: u8,
        );

        let target_color: Color = target_piece.get_color();

        match (current_color, target_color) {
            (Color::EMPTY, _) => { panic!("Error when calling report.include_step. Own color is empty."); },
            (Color::BLACK, Color::WHITE) | (Color::WHITE, Color::BLACK) => {
                // Rival position, returns false.
                if !no_rival {
                    // If above is true (used by pawn), ignores rival positions.
                    if target_piece.is_king() {
                        self.set_check();
                    }

                    if !self.search_checkmate{
                        // If this is true, we're only checking for checkmate. We won't use steps.
                        let step: Step = Step::new(
                            self.piece.clone(),     // current: Piece,
                            target_piece,           // target: Piece,
                        );
    
                        self.steps.push(step);
                    }
                }
                return false
            },
            (Color::BLACK, Color::BLACK) | (Color::WHITE, Color::WHITE) => {
                // Ally position, doesn't include anything and returns false.
                return false;
            }
            (_, Color::EMPTY) => {
                // Empty position, returns true.
                if !no_empty {
                    // If this is true (used by pawn), ignores empty spots.
                    if !self.search_checkmate{
                        // If this is true, we're only checking for checkmate. We won't allocate steps.
                    
                        let step: Step = Step::new(
                            self.piece.clone(),// current: Piece, 
                            target_piece,// target: Piece,
                        );


                        self.steps.push(step);
                    }
                }

                return true;
            }
        }
    }

    pub fn is_step_here(&self, step: &Step) -> bool {
        // Will not even count if the player is not the same.
        if self.piece != step.current_piece{ 
            return false;
        }

        for value in &self.steps{
            if value == step {
                return true;
            }
        }

        false
    }

    pub fn can_avoid_checkmate(
        &mut self,
        board: &mut Board,
    ) -> bool {
        // If any of the possible moves can clear out check status, then it isn't checkmate.
        for step in &mut self.steps{
            if step.can_avoid_checkmate(
                board,                              // board: &mut Board,
            ){
                return true;
            }
        }

        false
    }


    /// Use the row and column value to return the index in the board.
    /// This equation is simple, but if I miss something in it, the compiler will not notice.
    /// Which is why I turned into a stateless method. Less risky.
    pub fn col_row_to_index(col: u8, row: u8) -> u8 {
        return row * 8 + col
    }

    /// Check for the positions in a straight line according to go_right and go_down.
    /// Count up to max_steps and stops when it finds an ally or is outside the board.
    pub fn count_steps(
        &mut self,
        go_right: i8, 
        go_down: i8,
        max_steps: i8,
        board: &Board,
    ) {
        let start_col: u8 = self.piece.get_column();
        let start_row: u8 = self.piece.get_row();
        // let color: Color = self.piece.get_color();

        // Using i8 to reduce type casting in each computation.
        // Without this panic, a bug like this could be hard to find.
        if max_steps < 0 {
            panic!("Error in count_steps. Max_steps is negative: {}\n", max_steps);
        }
    
        // let mut col: i8;
        // let mut row: i8;
        for counter in 1..(max_steps + 1){
            let col: i8 = start_col as i8 + go_right * counter;
            let row: i8 = start_row as i8 + go_down * counter;

            if !self.include_step(
                board,                              // board: &Board, 
                col,                                // target_column: i8, 
                row,                                // target_row: i8, 
                false,                              // no_empty: bool, 
                false,                              // no_rival: bool,
            ){
                // Function above returns true only as it meets empty positions.
                // So if false, stop counting.
                break;
            }
        }
    }

    // Rook, Queen and King use this. Saves a lot of lines.
    pub fn apply_valid_ortogonal_positions(
        &mut self,
        max_steps: i8,
        board: &Board,
    ){
        // Count all steps available to the right of the piece.
        self.count_steps(
            1,                 // go_right: i8,
            0,                 // go_down: i8,
            max_steps,         // max_steps: i8,
            &board,            // board: &Board,
        );

        // Count all steps available to the left of the piece.
        self.count_steps(
            -1,                // go_right: i8,
            0,                 // go_down: i8,
            max_steps,         // max_steps: i8,
            &board,            // board: &Board,
        );

        // Count all steps available below the piece.
        self.count_steps(
            0,                 // go_right: i8,
            1,                 // go_down: i8,
            max_steps,         // max_steps: i8,
            &board,            // board: &Board,
        );

        // Count all the steps available above the piece.
        self.count_steps(
            0,                 // go_right: i8,
            -1,                // go_down: i8,
            max_steps,         // max_steps: i8,
            &board,            // board: &Board,
        );
    }

    // Bishop, Queen and King use this. Saves a lot of lines.
    pub fn apply_valid_diagonal_positions(
        &mut self, 
        max_steps: i8,
        board: &Board,
    ){
        // count steps available down right
        self.count_steps(
            1,                  // go_right: i8, 
            1,                  // go_down: i8, 
            max_steps,          // max_steps: i8, 
            board,              // board: &Board
        );

        // count steps available down left
        self.count_steps(
            -1,                 // go_right: i8, 
            1,                  // go_down: i8, 
            max_steps,          // max_steps: i8, 
            board,              // board: &Board
        );

        // count steps available up right
        self.count_steps(
            1,                  // go_right: i8, 
            -1,                 // go_down: i8, 
            max_steps,          // max_steps: i8, 
            board,              // board: &Board
        );

        // count steps available up left
        self.count_steps(
            -1,                 // go_right: i8, 
            -1,                 // go_down: i8, 
            max_steps,          // max_steps: i8, 
            board,              // board: &Board
        );
    }
}