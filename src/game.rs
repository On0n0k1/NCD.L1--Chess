
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

use near_sdk::serde::{Deserialize, Serialize};

use crate::{
    log,
    board::Board,
    movement::step::Step,
    player::{
        errors::ErrorResponse,
        players::Players,
    },
    pieces::piece::Piece,
};


// squares represent each position in the board.
// turn is how many movements the game has completed.
// player_turn: false: White, true: Black
// game_over: if game is still running. false means no moves can be done.
//
// Later will implement a list of all steps taken.
// That way we can revert to a previous turn, through player's request.

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game{
    squares: Vec<u8>,

    turn: u8,
    player_turn: bool,
    game_over: bool,
}

impl Default for Game{
    fn default() -> Self{
        // let squares: [u8; 64] = Board::get_default_board();
        let squares: Vec<u8> = Vec::from(Board::get_default_board());

        Game{
            squares,
            turn: 0,
            player_turn: false,
            game_over: false,
        }
    }
}


#[near_bindgen]
impl Game{

    #[private]
    fn handle_error(&mut self, error: ErrorResponse) -> String {
        let message: &str = match error{
            // If movement starts in a position that has no piece.
            ErrorResponse::NoPiece => {"No piece found in given position. "},
            // If movement starts in a position owned by a rival player
            ErrorResponse::RivalPiece => {"A rival player owns this piece. "},
            // If target position is invalid for given piece
            ErrorResponse::InvalidMove => {"Invalid move. "},
            // If King is under check.
            ErrorResponse::KingIsCheck => {"King is currently under check"},
            // If it's checkmate,
            ErrorResponse::CheckMate => {
                self.game_over = true;
                "CheckMate"
            },
            // If the game is already over.
            ErrorResponse::GameOver => {
                self.game_over = true;
                "Game is already over"
            },
        };

        String::from(message)
    }

    #[private]
    pub fn get_squares(&self) -> Vec<u8> {
        self.squares.clone()
    }

    #[private]
    pub fn get_turn(&self) -> u8 {
        self.turn.clone()
    }

    #[private]
    pub fn get_player_turn(&self) -> bool {
        self.player_turn.clone()
    }

    #[private]
    pub fn is_game_over(&self) -> bool {
        self.game_over.clone()
    }

    #[private]
    pub fn reset_game_game(&mut self) -> String{
        self.squares = Vec::from(Board::get_default_board());
        self.turn = 0;
        self.player_turn = false;
        self.game_over = false;

        String::from("Reset successful.")
    }


    #[private]
    pub fn get_game_status_game(&self) -> Self{
        let game_running: &str = match self.game_over{
            false => "Running",
            true => "Game Over",
        };

        let turn: u8 = self.turn.clone();

        let player: &str = match self.player_turn {
            false => "White",
            true => "Black",
        };

        log(&format!("Game Status: {}\nTurn: {}\nCurrent player: {}\n",
            game_running,
            turn,
            player,
        ));

        log(
            &format!("\nBoard:\n{}\n", 
                Board::get_board_string(
                    &self.squares[..],              // squares: &[u8; 64]
                )
            )
        );

        // let squares: [u8; 64] = self.squares.clone();
        let squares: Vec<u8> = self.squares.clone();
        let turn: u8 = self.turn. clone();
        let player_turn: bool = self.player_turn.clone();
        let game_over: bool = self.game_over.clone();

        Game{
            squares,
            turn,
            player_turn,
            game_over,
        }
    }


    #[private]
    pub fn get_board_game(&self) -> String {
        format!("\nBoard:\n{}\n",
            Board::get_board_string(
                &self.squares,                  // squares: &[u8; 64]
            )
        )
    }

    #[private]
    pub fn get_piece_name_game(&self, col: u8, row: u8) -> String {
        let index: u8 = row * 8 + col;
        
        self.get_piece_name_index_game(
            index,                              // index: u8,
        )
    }

    #[private]
    pub fn get_piece_name_index_game(&self, index: u8) -> String {
        let piece_number: u8 = self.squares[index as usize];

        let name: String = Piece::get_piece_name(
            piece_number,                       // piece_number: u8,
        );
        log(&name);

        name
    }


    #[private]
    pub fn move_to_game(
        &mut self,
        current: u8,
        target: u8,
    ) -> String {
        if self.game_over{
            return String::from("Game already over, check status or call reset.");
        }

        if (current > 63) || (target > 63) {
            return String::from("Invalid Arguments. Must be lower than 64.");
        }

        let current_player: bool = self.player_turn;
        let turn: u8 = self.turn;
        
        // let squares: [u8; 64] = self.squares.clone();
        let mut squares: [u8; 64] = [0; 64];
        for index in 0..self.squares.len(){
            squares[index] = self.squares[index];
        }

        log(&format!("Creating Board"));
        let mut board: Board = Board::new(
            squares,                            // squares: [u8; 64]
        );

        log(&format!("Creating Players"));
        let mut players: Players = Players::new(
            &board,                             // board: &Board, 
            current_player,                     // current_player: bool, 
            turn,                               // turn: u8,
        );

        // using env::log here just to clear linter warning
        env::log(&format!("Creating Step").as_bytes());
        let step: Step = Step::new_index(
            current,                            // current: u8,
            target,                             // target: u8,
            &board,                             // board: &Board,
        );

        log(&format!("Calling move"));
        match players.move_to(
            &mut board,                         // board: &mut Board, 
            step,                               // mut target: Step,
        ) {
            Result::Err(err) => {
                return self.handle_error(
                    err,                        // error: ErrorResponse,
                );
            },
            _ => {
                // Copy the values from the virtual board to the machine state one.
                self.squares.clear();
                for piece_number in board.get_board_array(){
                    self.squares.push(piece_number);
                }

                let (
                    player_turn, 
                    turn,
                ): (bool, u8) = players.get_turn_status();

                self.player_turn = player_turn;
                self.turn = turn;
            
                return String::from("Move successful.");
            }
        }
    }
}

