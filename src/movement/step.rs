use crate::{
    board::Board,
    pieces::{
        piece::{
            Color,
            Piece,
        },
        piece_ext::PieceExt,
    },
    player::player::Player,
};


/// This represents a single possible step for one piece.
/// We use it for testing for checkmate.
#[derive(Clone)]
pub struct Step{
    pub current_piece: Piece,
    pub target_piece: Piece,
    pub moved: bool,
}

impl PartialEq for Step{
    fn eq(&self, other: &Self) -> bool {
        let self_current_column: u8 = self.current_piece.get_column();
        let self_current_row: u8 = self.current_piece.get_row();

        let self_target_column: u8 = self.target_piece.get_column();
        let self_target_row: u8 = self.target_piece.get_row();

        let other_current_column: u8 = other.current_piece.get_column();
        let other_current_row: u8 = other.current_piece.get_row();

        let other_target_column: u8 = other.target_piece.get_column();
        let other_target_row: u8 = other.target_piece.get_row();

        (self_current_column == other_current_column)
        && (self_current_row == other_current_row)
        && (self_target_column == other_target_column)
        && (self_target_row == other_target_row)
    }
}

impl Step{
    pub fn new(
        current: Piece,
        target: Piece,
    ) -> Self{
        Step{
            current_piece: current,
            target_piece: target,
            moved: false,
        }
    }

    pub fn new_index(
        current: u8,
        target: u8,
        board: &Board,
    ) -> Self {
        let current_piece: Piece = board.get_piece_index(
            current,// index: u8,
        );
        
        let target_piece: Piece = board.get_piece_index(
            target,// index: u8,
        );

        Step{
            current_piece,
            target_piece,
            moved: false,
        }
    }

    pub fn commit_move(
        &mut self,
        board: &mut Board,
    ) {
        let start_col: u8 = self.current_piece.get_column();
        let start_row: u8 = self.current_piece.get_row();
        let piece_number: u8 = self.current_piece.get_piece_number();
        let end_col: u8 = self.target_piece.get_column();
        let end_row: u8 = self.target_piece.get_row();

        // Replace current position with an empty piece
        board.replace_piece(
            0,                              // piece_number: u8, 
            start_col,                      // col: u8,
            start_row,                      // row: u8,
        );

        // Replace target position with starting piece.
        board.replace_piece(
            piece_number,                   // piece_number: u8, 
            end_col,                        // col: u8,
            end_row,                        // row: u8,
        );

        // Step has concluded.
        self.moved = true;
    }

    pub fn revert_move(
        &mut self,
        board: &mut Board,
    ){
        let start_piece_number: u8 = self.current_piece.get_piece_number();
        let start_col: u8 = self.current_piece.get_column();
        let start_row: u8 = self.current_piece.get_row();

        let end_piece_number: u8 = self.target_piece.get_piece_number();
        let end_col: u8 = self.target_piece.get_column();
        let end_row: u8 = self.target_piece.get_row();

        // Replace starting position with starting piece.
        board.replace_piece(
            start_piece_number,             // piece_number: u8, 
            start_col,                      // col: u8,
            start_row,                      // row: u8,
        );

        // Replace target position with target piece.
        board.replace_piece(
            end_piece_number,               // piece_number: u8, 
            end_col,                        // col: u8,
            end_row,                        // row: u8,
        );

        // Step is still to be concluded.
        self.moved = false;
    }

    /// commit_finalize and revert_finalize does the same as commit_move and commit_revert, but to the final state of the board.
    /// Should be used after the movement is considered valid.
    pub fn commit_finalize(
        &mut self,
        board: &mut Board,
    ){
        let start_col: u8 = self.current_piece.get_column();
        let start_row: u8 = self.current_piece.get_row();
        let piece_number: u8 = self.current_piece.get_piece_number();
        let end_col: u8 = self.target_piece.get_column();
        let end_row: u8 = self.target_piece.get_row();

        board.finalize_movement(
            0,                              // piece_number: u8,
            start_col,                      // col: u8, 
            start_row,                      // row: u8,
        );

        board.finalize_movement(
            piece_number,                   // piece_number: u8,
            end_col,                        // col: u8, 
            end_row,                        // row: u8,
        );
    }

    pub fn revert_finalize(
        &mut self,
        board: &mut Board,
    ){
        let start_piece_number: u8 = self.current_piece.get_piece_number();
        let start_col: u8 = self.current_piece.get_column();
        let start_row: u8 = self.current_piece.get_row();

        let end_piece_number: u8 = self.target_piece.get_piece_number();
        let end_col: u8 = self.target_piece.get_column();
        let end_row: u8 = self.target_piece.get_row();

        board.finalize_movement(
            start_piece_number,             // piece_number: u8,
            start_col,                      // col: u8, 
            start_row,                      // row: u8,
        );

        board.finalize_movement(
            end_piece_number,               // piece_number: u8,
            end_col,                        // col: u8, 
            end_row,                        // row: u8,
        );

    }

    pub fn can_avoid_checkmate(&mut self, board: &mut Board) -> bool {
        let current_color: Color = self.current_piece.get_color();

        let rival_color: Color = match current_color {
            Color::EMPTY => panic!("Error in Step.can_avoid_checkmate. Current_color is Empty."),
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };

        self.commit_move(
            board,                          // board: &mut Board,
        );

        let mut rival_player: Player = Player::new(
            rival_color,                    // color: Color,
            board,                          // board: &Board,
        );

        // search_checkmate = true means it won't build any vectors, it will only check if it's checking.
        rival_player.build_reports(
            board,                          // board: &Board, 
            true,                           // search_checkmate: bool,
        );

        self.revert_move(
            board,                          // board: &mut Board,
        );

        // if this move (step) really clears out current player's check state, then next instruction will return false.
        return !rival_player.is_check();
    }
}

