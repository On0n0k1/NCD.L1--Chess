use crate::{
    log,
    board::Board,
    pieces::{
        piece::{
            Color,
            // Piece,
        },
        // piece_ext::PieceExt,
    },
    movement::{
        // report::Report,
        step::Step,
    },
    player::{
        errors::ErrorResponse,
        turn::Turn,
        player::Player,
    },
};


pub struct Players{
    black: Player,
    white: Player,
    turn: Turn,
}


impl Players{
    pub fn new(board: &Board, current_player: bool, turn: u8) -> Self{

        log(&format!("Creating Black Pieces"));
        let mut black: Player = Player::new(
            Color::BLACK,                   // color: Color, 
            board,                          // board: &Board,
        );

        log(&format!("Building Black Pieces"));
        black.build_reports(
            board,                          // board: &Board,
            false,                          // search_checkmate: bool,
        );

        log(&format!("Creating White Pieces"));
        let mut white: Player = Player::new(
            Color::WHITE,                   // color: Color,
            board,                          // board: &Board,
        );

        log(&format!("Building White Pieces"));
        white.build_reports(
            board,                          // board: &Board,
            false,                          // search_checkmate: bool
        );

        // let turn: Turn = Turn::new();
        let turn: Turn = Turn::new(
            current_player,                 // current_player: bool,
            turn,                           // turn: u8,
        );

        Players{
            black,
            white,
            turn,
        }
    }
    

    pub fn move_to(&mut self, board: &mut Board, mut target: Step) -> Result<(), ErrorResponse> {
        let player_color: Color = self.turn.get_current_player_color();
        
        let (current_player, other_player): (&mut Player, &mut Player) = match player_color{
            Color::EMPTY => panic!("Error in Players.move_to. Own color is Empty."),
            Color::BLACK => {
                let current_player = &mut self.black;
                let other_player = &mut self.white;

                log(&format!("Current Player is Black."));
                (current_player, other_player)
            },
            Color::WHITE => {
                let current_player = &mut self.white;
                let other_player = &mut self.black;


                log(&format!("Current Player is White"));
                (current_player, other_player)
            }
        };

        current_player.build_reports(
            board,                          // board: &Board,
            false,                          // search_checkmate: bool
        );

        // ? can be used in Result type enums.
        // If Result::Ok(value), use the value in the code.
        // If Result::Err(err), call "return Result::Err(err)".
        // Can only be used in functions that return the same type of Result.
        // 
        // is_step_valid checks every piece for positions it can go to,
        // if the piece can go there, returns true.
        if !current_player.is_step_valid(&target)?{
            return Result::Err(ErrorResponse::InvalidMove);
        }

        if !target.can_avoid_checkmate(
            board,                          // board: &mut Board,
        ){
            return Result::Err(ErrorResponse::KingIsCheck);
        };

        // If it reaches here, then the movement is valid and will not trigger a check own itself.
        // Commit the move so rival can see the change.
        target.commit_move(
            board,                          // board: &mut Board,
        );

        // Finalize the move to update squares (the array of bytes)
        target.commit_finalize(
            board,                          // board: &mut Board,
        );

        // Now we have to check if rival is under check. If he/she is, then we have to see if it's checkmate.

        // Build rival's movement reports.
        // search_checkmate = false means that it will store the positions available. 
        other_player.build_reports(
            board,                          // board: &Board,
            false,                          // search_checkmate: bool,
        );

        if other_player.is_check(){
            
            // Call function in step to check if making the move will cause itself to be under check.
            if !other_player.can_avoid_checkmate(
                board,// board: &mut Board,
            ) {
                // This means that there's no move that the rival player can take for saving themselves from check status
                // Checkmate
                return Result::Err(ErrorResponse::CheckMate);
            }
        }
        
        // Move was successful, so we store the step and
        // go to the next turn.
        self.turn.next_turn(
            // step list is not implemented yet
            // target,                         // next_step: Step,
        )?;


        // Movement went through without any issue
        Result::Ok(())
    }

    pub fn get_turn_status(&self) -> (bool, u8) {
        // player_turn: bool,
        // turn: u8,
        // squares: [u8; 64],
        let player_turn: bool = self.turn.get_current_player_boolean();
        let turn: u8 = self.turn.get_turn();
        // let squares: [u8; 64] = board.get_board_array();
        

        (player_turn, turn)
    }
}